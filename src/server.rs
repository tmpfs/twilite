use crate::{config::Config, routes};
use anyhow::Result;
use async_sqlite::Client;
use axum::{Extension, Router, routing::get};
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct ServerState {
    pub client: Client,
}

impl ServerState {
    pub fn get(&self, key: String) -> Option<String> {
        // let db = self.db.lock().unwrap();
        // if let Some(item) = db.get(&key) {
        //     Some(item.verifier.clone())
        // } else {
        //     None
        // }
        // todo!();
        println!("get state: {}", key);
        None
    }

    pub fn set(&self, key: String, value: String) {
        // let mut db = self.db.lock().unwrap();
        // db.insert(
        //     key,
        //     ItemOauthAxum {
        //         verifier: value,
        //         created_at: SystemTime::now(),
        //     },
        // );
        // todo!();
        println!("set state: {} = {}", key, value);
    }
}

pub struct Server;

impl Server {
    /// Start the server.
    pub async fn start(config: Config, client: Client, open: bool) -> Result<()> {
        let state = Arc::new(ServerState { client });

        tracing::info!(bind = %config.bind);

        let mut app = Router::new()
            .route("/login/github", get(github::login))
            .route("/api/github/callback", get(github::callback))
            .route("/", get(routes::home));
        cfg_if::cfg_if!(
            if #[cfg(debug_assertions)] {
                use tower_http::services::ServeDir;
                app = app.nest_service("/assets", ServeDir::new("./public"));
            } else {
                app = app.route("/assets/{*wildcard}", get(routes::assets));
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
    use super::ServerState;
    use axum::Extension;
    use axum::extract::Query;
    use axum::response::Redirect;
    use oauth_axum::providers::github::GithubProvider;
    use oauth_axum::{CustomProvider, OAuthClient};
    use std::sync::Arc;

    #[derive(Clone, serde::Deserialize)]
    pub struct QueryAxumCallback {
        pub code: Option<String>,
        pub state: Option<String>,
    }

    pub async fn login(Extension(state): Extension<Arc<ServerState>>) -> Redirect {
        let auth_url = create_url(state).await;
        Redirect::temporary(&auth_url)
    }

    fn get_client() -> CustomProvider {
        GithubProvider::new(
            std::env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must be set"),
            std::env::var("GITHUB_SECRET").expect("GITHUB_SECRET must be set"),
            "http://localhost:8776/api/github/callback".to_string(),
        )
    }

    async fn create_url(state: Arc<ServerState>) -> String {
        let state_oauth = get_client()
            .generate_url(Vec::from(["read:user".to_string()]), |state_e| async move {
                state.set(state_e.state, state_e.verifier);
            })
            .await
            .ok()
            .unwrap()
            .state
            .unwrap();

        state_oauth.url_generated.unwrap()
    }

    pub async fn callback(
        Extension(state): Extension<Arc<ServerState>>,
        Query(queries): Query<QueryAxumCallback>,
    ) -> String {
        // println!("{:?}", state.get_all_items());

        if let (Some(oauth_code), Some(oauth_state)) = (queries.code, queries.state) {
            let item = state.get(oauth_state.clone());
            get_client()
                .generate_token(oauth_code, item.unwrap())
                .await
                .ok()
                .unwrap()
        } else {
            "Cancelled".to_string()
        }
    }
}
