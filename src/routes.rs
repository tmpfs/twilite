use crate::{
    error::ServerError,
    helpers::{html_to_text, sanitize_html},
    server::ServerState,
};
use axum::{
    Extension, Json,
    extract::{Multipart, Path},
    http::{HeaderMap, StatusCode, Uri, header},
    response::{Html, IntoResponse, Redirect, Response},
};
use rust_embed::RustEmbed;
use sql_query_builder as sql;
use std::sync::Arc;
use time::{UtcDateTime, format_description::well_known::Rfc3339};
use uuid::Uuid;

#[derive(RustEmbed)]
#[folder = "app/out"]
struct Assets;

pub async fn api_delete_page(
    Extension(state): Extension<Arc<ServerState>>,
    Path(page_uuid): Path<Uuid>,
) -> Result<Response, ServerError> {
    let query = sql::Delete::new()
        .delete_from("pages")
        .where_clause("page_uuid = ?1");
    let client = state.client.lock().await;
    let content: Result<usize, async_sqlite::Error> = client
        .conn(move |conn| {
            let mut stmt = conn.prepare_cached(&query.as_string())?;
            stmt.execute([page_uuid.to_string()])
        })
        .await;
    match content {
        Ok(_affected_rows) => Ok(StatusCode::OK.into_response()),
        Err(async_sqlite::Error::Rusqlite(async_sqlite::rusqlite::Error::QueryReturnedNoRows)) => {
            Ok(StatusCode::NOT_FOUND.into_response())
        }
        Err(e) => Err(e.into()),
    }
}

pub async fn api_select_page_content(
    Extension(state): Extension<Arc<ServerState>>,
    headers: HeaderMap,
    Path(page_name): Path<String>,
) -> Result<Response, ServerError> {
    if let Some(accept) = headers.get("accept") {
        if accept == "application/json" {
            api_select_page_json(state, page_name).await
        } else if accept == "text/html" {
            api_select_page_html(state, page_name).await
        } else {
            Ok((StatusCode::UNSUPPORTED_MEDIA_TYPE, "Unsupported media type").into_response())
        }
    } else {
        Ok((StatusCode::NOT_ACCEPTABLE, "Unsupported Accept header").into_response())
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResponse {
    page_uuid: Uuid,
    page_name: String,
    page_content: String,
}

async fn api_select_page_json(
    state: Arc<ServerState>,
    page_name: String,
) -> Result<Response, ServerError> {
    let query = sql::Select::new()
        .select("page_uuid, page_name, page_content")
        .from("pages")
        .where_clause("page_name = ?1");
    let client = state.client.lock().await;
    let content: Result<PageResponse, async_sqlite::Error> = client
        .conn(move |conn| {
            let mut stmt = conn.prepare_cached(&query.as_string())?;
            stmt.query_row([page_name], |row| {
                let page_uuid = row.get::<_, String>("page_uuid")?;
                let page_uuid = page_uuid.parse().unwrap();
                Ok(PageResponse {
                    page_uuid,
                    page_name: row.get("page_name")?,
                    page_content: row.get("page_content")?,
                })
            })
        })
        .await;
    match content {
        Ok(page_content) => Ok(Json(page_content).into_response()),
        Err(async_sqlite::Error::Rusqlite(async_sqlite::rusqlite::Error::QueryReturnedNoRows)) => {
            Ok(StatusCode::NOT_FOUND.into_response())
        }
        Err(e) => Err(e.into()),
    }
}

async fn api_select_page_html(
    state: Arc<ServerState>,
    page_name: String,
) -> Result<Response, ServerError> {
    let query = sql::Select::new()
        .select("page_content")
        .from("pages")
        .where_clause("page_name = ?1");
    let client = state.client.lock().await;
    let content: Result<String, async_sqlite::Error> = client
        .conn(move |conn| {
            let mut stmt = conn.prepare_cached(&query.as_string())?;
            stmt.query_row([page_name], |row| row.get(0))
        })
        .await;
    match content {
        Ok(page_content) => Ok(Html(page_content).into_response()),
        Err(async_sqlite::Error::Rusqlite(async_sqlite::rusqlite::Error::QueryReturnedNoRows)) => {
            Ok(StatusCode::NOT_FOUND.into_response())
        }
        Err(e) => Err(e.into()),
    }
}

pub async fn api_insert_page(
    Extension(state): Extension<Arc<ServerState>>,
    mut multipart: Multipart,
) -> Result<Response, ServerError> {
    let mut page_name = None;
    let mut page_content = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let value = field.text().await.unwrap();

        match name.as_str() {
            "pageName" => page_name = Some(value),
            "pageContent" => page_content = Some(value),
            _ => {}
        }
    }

    let (Some(page_name), Some(page_content)) = (page_name, page_content) else {
        return Ok(StatusCode::BAD_REQUEST.into_response());
    };

    let query = sql::Insert::new()
        .insert_into(
            "pages (created_at, updated_at, page_uuid, page_name, page_content, page_text)",
        )
        .values("(?1, ?2, ?3, ?4, ?5, ?6)");

    let now = UtcDateTime::now();
    let created_at = now.format(&Rfc3339)?;
    let updated_at = now.format(&Rfc3339)?;
    let page_uuid = Uuid::new_v4();
    let page_content = sanitize_html(&page_content);
    let page_text = html_to_text(&page_content);
    let client = state.client.lock().await;
    match client
        .conn(move |conn| {
            let mut stmt = conn.prepare_cached(&query.as_string())?;
            stmt.execute((
                created_at,
                updated_at,
                page_uuid.to_string(),
                page_name,
                page_content,
                page_text,
            ))?;
            Ok(())
        })
        .await
    {
        Ok(_) => Ok(StatusCode::OK.into_response()),
        Err(e) => match e {
            async_sqlite::Error::Rusqlite(async_sqlite::rusqlite::Error::SqliteFailure(err, _)) => {
                if err.code == async_sqlite::rusqlite::ErrorCode::ConstraintViolation {
                    Ok(StatusCode::CONFLICT.into_response())
                } else {
                    Err(e.into())
                }
            }
            _ => Err(e.into()),
        },
    }
}

pub async fn api_update_page(
    Extension(state): Extension<Arc<ServerState>>,
    Path(page_uuid): Path<Uuid>,
    mut multipart: Multipart,
) -> Result<Response, ServerError> {
    let mut page_name = None;
    let mut page_content = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let value = field.text().await.unwrap();

        match name.as_str() {
            "pageName" => page_name = Some(value),
            "pageContent" => page_content = Some(value),
            _ => {}
        }
    }

    let (Some(page_name), Some(page_content)) = (page_name, page_content) else {
        return Ok(StatusCode::BAD_REQUEST.into_response());
    };

    let query = sql::Update::new()
        .update("pages")
        .set("updated_at = ?1, page_name = ?2, page_content = ?3")
        .where_clause("page_uuid = ?4");

    let now = UtcDateTime::now();
    let updated_at = now.format(&Rfc3339)?;
    let page_content = sanitize_html(&page_content);
    let client = state.client.lock().await;
    client
        .conn(move |conn| {
            let mut stmt = conn.prepare_cached(&query.as_string())?;
            stmt.execute((updated_at, page_name, page_content, page_uuid.to_string()))?;
            Ok(())
        })
        .await?;

    Ok(StatusCode::OK.into_response())
}

pub async fn home() -> impl IntoResponse {
    Redirect::permanent("/index.html")
}

pub async fn assets(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches("/").to_owned();
    if path.ends_with('/') {
        path.push_str("index.html");
    }
    match Assets::get(&path) {
        Some(content) => {
            let body = content.data;
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(body.into())
                .unwrap()
        }
        None => (StatusCode::NOT_FOUND, "Not Found").into_response(),
    }
}

pub async fn asset_new_index() -> impl IntoResponse {
    let path = "new/index.html";
    serve_embedded(path).await
}

pub async fn asset_edit_index() -> impl IntoResponse {
    let path = "edit/index.html";
    serve_embedded(path).await
}

pub async fn asset_wiki_index() -> impl IntoResponse {
    let path = "wiki/index.html";
    serve_embedded(path).await
}

async fn serve_embedded(path: &str) -> impl IntoResponse {
    match Assets::get(path) {
        Some(content) => {
            let body = content.data;
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(body.into())
                .unwrap()
        }
        None => (StatusCode::NOT_FOUND, "Not Found").into_response(),
    }
}
