use crate::{error::ServerError, helpers::sanitize_html, server::ServerState};
use axum::{
    Extension,
    extract::{Multipart, Path},
    http::{StatusCode, Uri, header},
    response::{Html, IntoResponse, Redirect, Response},
};
use rust_embed::RustEmbed;
use sql_query_builder as sql;
use std::sync::Arc;
use time::{UtcDateTime, format_description::well_known::Rfc3339};

#[derive(RustEmbed)]
#[folder = "app/out"]
struct Assets;

pub async fn api_select_page(
    Extension(state): Extension<Arc<ServerState>>,
    Path(page_name): Path<String>,
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
        .insert_into("pages (created_at, updated_at, page_name, page_content)")
        .values("(?1, ?2, ?3, ?4)");

    let now = UtcDateTime::now();
    let created_at = now.format(&Rfc3339)?;
    let updated_at = now.format(&Rfc3339)?;
    let page_content = sanitize_html(&page_content);
    let client = state.client.lock().await;
    client
        .conn(move |conn| {
            let mut stmt = conn.prepare_cached(&query.as_string())?;
            stmt.execute((created_at, updated_at, page_name, page_content))?;
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

pub async fn asset_wiki_index() -> impl IntoResponse {
    let path = "wiki/index.html";
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
