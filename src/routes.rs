use axum::{
    http::{StatusCode, Uri, header},
    response::{IntoResponse, Redirect, Response},
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "public/"]
struct Assets;

// async fn index() -> impl IntoResponse {
//     // Show "Login with GitHub" button
//     r#"
//     <html>
//         <body>
//             <a href="/login/github">
//                 <button>Login with GitHub</button>
//             </a>
//         </body>
//     </html>
//     "#
// }

pub async fn home() -> impl IntoResponse {
    Redirect::permanent("/index.html")
}

/*
            textarea id = "editor" {}
            script {
                (PreEscaped(r#"
                const easyMDE = new EasyMDE({element: document.getElementById("editor")});
                "#))
            }
*/

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
