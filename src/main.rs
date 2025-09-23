use anyhow::Result;
use async_sqlite::{ClientBuilder, JournalMode};
use axum::{Router, routing::get};
use maud::{Markup, html};
use wikilite::migrations;

async fn home() -> Markup {
    html! {
        h1 { "Wikilite" }
    }
}

async fn run() -> Result<()> {
    let mut db_client = ClientBuilder::new()
        .path("wikilite.sqlite3")
        .journal_mode(JournalMode::Wal)
        .open()
        .await?;

    migrations::migrate_client(&mut db_client).await?;

    let app = Router::new().route("/", get(home));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{}", e);
        tracing::error!("{}", e);
        std::process::exit(1);
    }
}
