use anyhow::{Context, Result};
use async_sqlite::{ClientBuilder, JournalMode};
use clap::Parser;
use std::path::{Path, PathBuf};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use wikilite::{config::*, migrations, server::Server};

const DEFAULT_LOG_LEVEL: &str = "wikilite=info";

/// Initialize the tracing subscriber.
#[cfg(debug_assertions)]
pub fn init_subscriber(logs_dir: &Path, name: &str, log_level: String) -> Result<()> {
    let logfile = RollingFileAppender::new(Rotation::DAILY, logs_dir, name);
    let env_layer = tracing_subscriber::EnvFilter::new(log_level);

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
        let content = std::fs::read(&config_path)
            .with_context(|| format!("unsable to read config file {}", config_path.display()))?;
        let content: Config = toml::from_slice(&content)?;
        content
    } else {
        Config::default()
    };

    let env_path = if let Some(env) = &config.env {
        env.clone()
    } else {
        std::env::current_dir()?.join(".env")
    };

    dotenv::from_path(&env_path)
        .with_context(|| format!("unable to load .env file from: {}", env_path.display()))?;

    tracing::info!(database = %config.database.path);

    let mut db_client = ClientBuilder::new()
        .path(&config.database.path)
        .journal_mode(JournalMode::Wal)
        .open()
        .await
        .with_context(|| format!("unable to initialize database: {}", &config.database.path))?;

    migrations::migrate_client(&mut db_client).await?;
    Server::start(config, db_client, args.open).await
}

#[tokio::main]
async fn main() {
    let logs_dir = std::env::var("WIKILITE_LOGS_DIR")
        .ok()
        .map(PathBuf::from)
        .unwrap_or(PathBuf::from("logs"));

    if !logs_dir.exists() {
        std::fs::create_dir_all(&logs_dir).expect("logs directory to exist");
    }

    let log_file_name = std::env::var("WIKILITE_LOG_FILE_NAME")
        .ok()
        .unwrap_or("wikilite.log".to_owned());

    init_subscriber(
        &logs_dir,
        &log_file_name,
        std::env::var("RUST_LOG")
            .ok()
            .unwrap_or(DEFAULT_LOG_LEVEL.to_string()),
    )
    .expect("to initialize tracing");

    if let Err(e) = run().await {
        // eprintln!("{}", e);
        tracing::error!("{}", e);
        std::process::exit(1);
    }
}
