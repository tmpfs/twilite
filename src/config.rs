use clap::Parser;
use serde::Deserialize;
use std::{net::SocketAddr, path::PathBuf};

#[derive(Parser)]
#[clap(name = "wikilite", author, version, about, long_about = None)]
pub struct WikiLiteCli {
    /// Configuration file.
    #[clap(short, long)]
    pub config: Option<PathBuf>,

    /// Open on startup.
    #[clap(short, long)]
    pub open: bool,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub bind: SocketAddr,
    pub database: Database,
    pub logs: Logs,
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
pub struct Database {
    pub path: String,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            path: String::from("wikilite.sqlite3"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Logs {
    pub log_level: Option<String>,
    pub logs_dir: PathBuf,
    pub log_file_name: String,
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
