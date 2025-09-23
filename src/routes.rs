use axum::{
    http::{StatusCode, Uri, header},
    response::{IntoResponse, Response},
};
use maud::{Markup, html};

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "public/"]
struct Assets;

pub async fn home() -> Markup {
    html! {
        h1 { "Wikilite" }
    }
}

pub async fn assets(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches("/assets/");
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
