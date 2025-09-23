use axum::{
    http::{StatusCode, Uri, header},
    response::{IntoResponse, Response},
};
use maud::{Markup, PreEscaped, html};
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

pub async fn home() -> Markup {
    html! {
        head {
            link rel = "stylesheet" href = "/assets/easymde.min.css";
            script src="/assets/easymde.min.js" {}
        }
        body {
            h1 { "Wikilite" }
            a href="/login/github" {
                button {
                    "Login with Github"
                }
            }
        }
    }
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
