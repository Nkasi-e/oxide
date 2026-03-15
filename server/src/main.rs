use std::{net::SocketAddr, sync::Arc};

use server::{
    AppState,
    api::routes,
    config::AppConfig,
    db::{connection, repositories::DbRepository},
    error::AppResult,
    github::client::GitHubClient,
    services::{
        contributor_service::ContributorService, galaxy_service::GalaxyService, repo_service::RepoService,
    },
    worker::{ingestion::IngestionWorker, scheduler::IngestionScheduler},
};
use tokio::sync::mpsc;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> AppResult<()> {
    init_tracing();
    let config = AppConfig::from_env()?;

    let pool = connection::connect(&config).await?;
    let db_repo = Arc::new(DbRepository::new(pool));
    let github_client = Arc::new(GitHubClient::new(config.github_token.clone())?);

    let repo_service = Arc::new(RepoService::new(db_repo.clone(), github_client.clone()));
    let contributor_service = Arc::new(ContributorService::new(db_repo.clone()));
    let galaxy_service = Arc::new(GalaxyService::new(db_repo.clone()));

    let (tx, rx) = mpsc::channel(config.worker_queue_capacity);
    let scheduler = Arc::new(IngestionScheduler::new(tx));

    let worker = IngestionWorker::new(
        rx,
        github_client,
        repo_service.clone(),
        contributor_service,
        galaxy_service.clone(),
        db_repo,
        config.galaxy_version,
    );
    tokio::spawn(worker.run());

    let state = AppState {
        repo_service,
        galaxy_service,
        ingestion_scheduler: scheduler,
    };

    let app = routes::router(state).layer(TraceLayer::new_for_http());
    let addr: SocketAddr = format!("{}:{}", config.server_host, config.server_port)
        .parse()
        .map_err(|e| server::error::AppError::Other(format!("invalid bind address: {e}")))?;

    let listener = tokio::net::TcpListener::bind(addr).await.map_err(|e| {
        server::error::AppError::Other(format!("failed to bind listener on {addr}: {e}"))
    })?;

    info!("github-galaxy backend listening on {}", addr);
    axum::serve(listener, app)
        .await
        .map_err(|e| server::error::AppError::Other(format!("server runtime failure: {e}")))?;

    Ok(())
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer().compact())
        .init();
}
