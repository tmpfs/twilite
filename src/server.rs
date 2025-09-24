use crate::{config::Config, routes};
use anyhow::Result;
use async_sqlite::Client;
use axum::body::Body;
use axum::http::{HeaderValue, Request, header};
use axum::middleware::{self, Next};
use axum::response::Response;
use axum::{
    Extension, Router,
    routing::{get, post},
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use tower::ServiceBuilder;

#[derive(Clone, Debug)]
pub struct ItemOauthAxum {
    pub verifier: String,
    pub created_at: SystemTime,
}

#[derive(Clone)]
pub struct ServerState {
    pub client: Client,
    pub auth_db: Arc<Mutex<HashMap<String, ItemOauthAxum>>>,
}

impl ServerState {
    pub async fn get(&self, key: String) -> Option<String> {
        let db = self.auth_db.lock().unwrap();
        db.get(&key).map(|i| i.verifier.clone())
    }

    pub async fn set(&self, key: String, value: String) {
        let mut db = self.auth_db.lock().unwrap();
        db.insert(
            key,
            ItemOauthAxum {
                verifier: value,
                created_at: SystemTime::now(),
            },
        );
    }
}

/// Don't cache static assets in debug mode.
async fn set_static_cache_control(request: Request<Body>, next: Next) -> Response {
    let mut response = next.run(request).await;
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
        .headers_mut()
        .insert(header::PRAGMA, HeaderValue::from_static("no-cache"));
    response
        .headers_mut()
        .insert(header::EXPIRES, HeaderValue::from_static("0"));
    response
}

pub struct Server;

impl Server {
    /// Start the server.
    pub async fn start(config: Config, client: Client, open: bool) -> Result<()> {
        let state = Arc::new(ServerState {
            client,
            auth_db: Arc::new(Mutex::new(HashMap::new())),
        });

        tracing::info!(bind = %config.bind);

        let mut app = Router::new()
            .route("/login/github", get(github::login))
            .route("/api/page", post(routes::api_page))
            .route("/api/github/callback", get(github::callback))
            .route("/", get(routes::home));
        cfg_if::cfg_if!(
            if #[cfg(debug_assertions)] {
                use tower_http::services::ServeDir;
                app = app.fallback_service(
                    ServiceBuilder::new()
                        .layer(middleware::from_fn(set_static_cache_control))
                        .service(
                            ServeDir::new("./app/out")
                        )
                    );
            } else {
                app = app.route("/{*wildcard}", get(routes::assets));
            }
        );

        app = app.layer(Extension(state.clone()));

        let listener = tokio::net::TcpListener::bind(config.bind).await?;
        tracing::info!("listening on {}", listener.local_addr().unwrap());
        if open {
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_millis(250)).await;
                open::that("http://localhost:8776").expect("to open URL");
            });
        }
        axum::serve(listener, app).await?;

        Ok(())
    }
}

mod github {
    use crate::error::ServerError;

    use super::ServerState;
    use anyhow::Result;
    use axum::Extension;
    use axum::extract::Query;
    use axum::response::{IntoResponse, Redirect, Response};
    use oauth_axum::providers::github::GithubProvider;
    use oauth_axum::{CustomProvider, OAuthClient};
    use std::sync::Arc;

    #[derive(Clone, serde::Deserialize)]
    pub struct OauthCallback {
        pub code: Option<String>,
        pub state: Option<String>,
    }

    pub async fn login(
        Extension(state): Extension<Arc<ServerState>>,
    ) -> Result<Redirect, ServerError> {
        let auth_url = create_url(state).await?;
        Ok(Redirect::temporary(&auth_url))
    }

    fn get_client() -> CustomProvider {
        GithubProvider::new(
            std::env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must be set"),
            std::env::var("GITHUB_SECRET").expect("GITHUB_SECRET must be set"),
            "http://localhost:8776/api/github/callback".to_string(),
        )
    }

    async fn create_url(state: Arc<ServerState>) -> Result<String, ServerError> {
        let state_oauth = get_client()
            .generate_url(
                Vec::from(["read:user".to_string(), "user:email".to_string()]),
                |state_e| async move {
                    state.set(state_e.state, state_e.verifier).await;
                },
            )
            .await?
            .state
            .ok_or(ServerError::GenerateOauthUrl)?;
        state_oauth
            .url_generated
            .ok_or(ServerError::NoGeneratedOauthUrl)
    }

    pub async fn callback(
        Extension(state): Extension<Arc<ServerState>>,
        Query(queries): Query<OauthCallback>,
    ) -> Result<Response, ServerError> {
        if let (Some(oauth_code), Some(oauth_state)) = (queries.code, queries.state) {
            let item = state.get(oauth_state.clone()).await;
            let token = get_client()
                .generate_token(oauth_code, item.unwrap())
                .await?;
            println!("Authorized...{}", token);
            Ok(Redirect::temporary("/").into_response())
        } else {
            Ok(Redirect::temporary("/").into_response())
        }
    }
}
