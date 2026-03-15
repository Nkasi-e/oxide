use serde::Deserialize;
use std::env;

use crate::error::AppError;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server_host: String,
    pub server_port: u16,
    pub database_url: String,
    pub github_token: Option<String>,
    pub worker_queue_capacity: usize,
    pub galaxy_version: i32,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
        // Load a `.env` file from the current working directory (e.g. `server/`).
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| AppError::Other("DATABASE_URL is not set".to_string()))?;

        let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| default_host());
        let server_port = env::var("SERVER_PORT")
            .ok()
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or_else(|| default_port());

        let github_token = env::var("GITHUB_TOKEN").ok();

        let worker_queue_capacity = env::var("WORKER_QUEUE_CAPACITY")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or_else(|| default_worker_capacity());

        let galaxy_version = env::var("GALAXY_VERSION")
            .ok()
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or_else(|| default_galaxy_version());

        Ok(AppConfig {
            server_host,
            server_port,
            database_url,
            github_token,
            worker_queue_capacity,
            galaxy_version,
        })
    }
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_worker_capacity() -> usize {
    512
}

fn default_galaxy_version() -> i32 {
    1
}
