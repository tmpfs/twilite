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
    pub env: Option<PathBuf>,
    pub database: Database,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind: "0.0.0.0:8776".parse().unwrap(),
            env: None,
            database: Database::default(),
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
            path: String::from("data/wikilite.sqlite3"),
        }
    }
}
