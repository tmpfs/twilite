use anyhow::{Context, Result};
use async_sqlite::{ClientBuilder, JournalMode};
use axum::{Router, routing::get};
use clap::Parser;
use std::{path::Path, time::Duration};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use wikilite::{config::*, migrations, routes};

const DEFAULT_LOG_LEVEL: &str = "wikilite=info";

/// Initialize the tracing subscriber.
#[cfg(debug_assertions)]
pub fn init_subscriber(logs_dir: &Path, name: &str, log_level: String) -> Result<()> {
    let logfile = RollingFileAppender::new(Rotation::DAILY, logs_dir, name);
    let env_layer =
        tracing_subscriber::EnvFilter::new(std::env::var("RUST_LOG").unwrap_or(log_level));

    let file_layer = tracing_subscriber::fmt::layer()
        .with_file(false)
        .with_line_number(false)
        .with_ansi(false)
        .json()
        .with_writer(logfile);

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_file(false)
        .with_line_number(false)
        .with_writer(std::io::stderr)
        .with_target(false);

    tracing_subscriber::registry()
        .with(env_layer)
        .with(fmt_layer)
        .with(file_layer)
        .try_init()?;

    Ok(())
}

async fn run() -> Result<()> {
    let args = WikiLiteCli::parse();

    let config = if let Some(config_path) = args.config {
        let content = std::fs::read(config_path)?;
        let content: Config = toml::from_slice(&content)?;
        content
    } else {
        Config::default()
    };

    if !config.logs.logs_dir.exists() {
        std::fs::create_dir_all(&config.logs.logs_dir)
            .with_context(|| "failed to create logs directory")?;
    }

    init_subscriber(
        &config.logs.logs_dir,
        &config.logs.log_file_name,
        config
            .logs
            .log_level
            .unwrap_or(DEFAULT_LOG_LEVEL.to_string()),
    )?;

    let mut db_client = ClientBuilder::new()
        .path(config.database.path)
        .journal_mode(JournalMode::Wal)
        .open()
        .await
        .with_context(|| "failed to initialize database")?;

    migrations::migrate_client(&mut db_client).await?;

    tracing::info!(bind = %config.bind);

    let mut app = Router::new().route("/", get(routes::home));
    cfg_if::cfg_if!(
        if #[cfg(debug_assertions)] {
            use tower_http::services::ServeDir;
            app = app.nest_service("/assets", ServeDir::new("./public"));
        } else {
            app = app.route("/assets/{*wildcard}", get(routes::assets));
        }
    );

    let listener = tokio::net::TcpListener::bind(config.bind).await?;
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    if args.open {
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(250)).await;
            open::that("http://localhost:8776").expect("to open URL");
        });
    }
    axum::serve(listener, app).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        // eprintln!("{}", e);
        tracing::error!("{}", e);
        std::process::exit(1);
    }
}
