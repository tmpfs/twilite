use crate::{
    entity::page::{PageEntity, PageResponse},
    error::ServerError,
    server::ServerState,
};
use axum::{
    Extension, Json,
    body::Bytes,
    extract::{Multipart, Path},
    http::{HeaderMap, StatusCode, Uri, header},
    response::{Html, IntoResponse, Redirect, Response},
};
use rust_embed::RustEmbed;
use sql_query_builder as sql;
use std::sync::Arc;
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

pub async fn api_recent_pages(
    Extension(state): Extension<Arc<ServerState>>,
    headers: HeaderMap,
) -> Result<Response, ServerError> {
    if let Some(accept) = headers.get("accept") {
        if accept == "application/json" {
            // api_select_page_json(state, page_name).await
            todo!();
        } else if accept == "text/html" {
            // api_select_page_html(state, page_name).await
            todo!();
        } else {
            Ok((StatusCode::UNSUPPORTED_MEDIA_TYPE, "Unsupported media type").into_response())
        }
    } else {
        Ok((StatusCode::NOT_ACCEPTABLE, "Unsupported Accept header").into_response())
    }
}

async fn api_select_page_json(
    state: Arc<ServerState>,
    page_name: String,
) -> Result<Response, ServerError> {
    let client = state.client.lock().await;
    match PageEntity::find_by_name(&client, page_name).await {
        Ok(entity) => {
            let response: PageResponse = entity.into();
            Ok(Json(response).into_response())
        }
        Err(e) => Err(e),
    }
}

async fn api_select_page_html(
    state: Arc<ServerState>,
    page_name: String,
) -> Result<Response, ServerError> {
    let client = state.client.lock().await;
    match PageEntity::find_by_name(&client, page_name).await {
        Ok(entity) => Ok(Html(entity.page_content).into_response()),
        Err(e) => Err(e),
    }
}

pub async fn api_insert_page(
    Extension(state): Extension<Arc<ServerState>>,
    mut multipart: Multipart,
) -> Result<Response, ServerError> {
    let mut page_name = None;
    let mut page_content = None;
    let mut uploads: Vec<(Option<String>, Option<String>, Bytes)> = vec![];

    while let Some(field) = multipart.next_field().await.unwrap() {
        println!("{}", field.name().unwrap());
        match field.name().unwrap() {
            "pageName" => page_name = Some(field.text().await.unwrap()),
            "pageContent" => page_content = Some(field.text().await.unwrap()),
            "uploads" => {
                uploads.push((
                    field.file_name().map(|s| s.to_owned()),
                    field.content_type().map(|s| s.to_owned()),
                    field.bytes().await?,
                ));
            }
            _ => {}
        }
    }

    let (Some(page_name), Some(page_content)) = (page_name, page_content) else {
        return Ok(StatusCode::BAD_REQUEST.into_response());
    };

    println!(
        "Server got files: {:#?}",
        uploads.iter().map(|u| &u.0).collect::<Vec<_>>()
    );

    let client = state.client.lock().await;
    match PageEntity::add(&client, page_name, page_content).await {
        Ok(_) => Ok(StatusCode::OK.into_response()),
        Err(e) => Err(e),
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

    let client = state.client.lock().await;
    PageEntity::edit(&client, page_uuid, page_name, page_content)
        .await
        .map(|_| StatusCode::OK.into_response())
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
