pub mod api;
pub mod config;
pub mod db;
pub mod error;
pub mod galaxy;
pub mod github;
pub mod services;
pub mod worker;

use std::sync::Arc;

use services::galaxy_service::GalaxyService;
use services::repo_service::RepoService;
use worker::scheduler::IngestionScheduler;

#[derive(Clone)]
pub struct AppState {
    pub repo_service: Arc<RepoService>,
    pub galaxy_service: Arc<GalaxyService>,
    pub ingestion_scheduler: Arc<IngestionScheduler>,
}
