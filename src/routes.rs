use axum::{
    extract::Multipart,
    http::{StatusCode, Uri, header},
    response::{IntoResponse, Redirect, Response},
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "public/"]
struct Assets;

/*
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageForm {
    page_name: String,
    page_content: String,
}
*/

pub async fn api_page(mut multipart: Multipart) -> impl IntoResponse {
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

    format!(
        "Got pageName = {:?}, pageContent = {:?}",
        page_name, page_content
    )
}

pub async fn home() -> impl IntoResponse {
    Redirect::permanent("/index.html")
}

pub async fn assets(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches("/");
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
