use anyhow::{Context, Result};
use async_sqlite::{ClientBuilder, JournalMode};
use axum::{Router, routing::get};
use clap::Parser;
use serde::Deserialize;
use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use wikilite::{migrations, routes};

const DEFAULT_LOG_LEVEL: &str = "wikilite=info";

#[derive(Parser)]
struct WikiLite {
    /// Configuration file.
    #[clap(short, long)]
    config: Option<PathBuf>,

    /// Open on startup.
    #[clap(short, long)]
    open: bool,
}

#[derive(Debug, Deserialize)]
struct Config {
    bind: SocketAddr,
    database: Database,
    logs: Logs,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind: "0.0.0.0:8776".parse().unwrap(),
            database: Database::default(),
            logs: Logs::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Database {
    path: String,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            path: String::from("wikilite.sqlite3"),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Logs {
    log_level: Option<String>,
    logs_dir: PathBuf,
    log_file_name: String,
}

impl Default for Logs {
    fn default() -> Self {
        Self {
            log_level: None,
            logs_dir: PathBuf::from("logs"),
            log_file_name: String::from("wikilite.log"),
        }
    }
}

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
    let args = WikiLite::parse();

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

    let app = Router::new()
        .route("/", get(routes::home))
        .route("/assets/{*wildcard}", get(routes::assets));
    let listener = tokio::net::TcpListener::bind(config.bind).await?;
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
