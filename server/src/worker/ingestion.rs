use std::sync::Arc;

use tokio::sync::mpsc;
use tracing::{error, info};

use crate::{
    db::repositories::{DbRepository, RepoLanguageInput},
    github::client::GitHubClient,
    services::{contributor_service::ContributorService, galaxy_service::GalaxyService, repo_service::RepoService},
    worker::scheduler::IngestionJob,
};

pub struct IngestionWorker {
    rx: mpsc::Receiver<IngestionJob>,
    github: Arc<GitHubClient>,
    repo_service: Arc<RepoService>,
    contributor_service: Arc<ContributorService>,
    galaxy_service: Arc<GalaxyService>,
    db: Arc<DbRepository>,
    galaxy_version: i32,
}

impl IngestionWorker {
    pub fn new(
        rx: mpsc::Receiver<IngestionJob>,
        github: Arc<GitHubClient>,
        repo_service: Arc<RepoService>,
        contributor_service: Arc<ContributorService>,
        galaxy_service: Arc<GalaxyService>,
        db: Arc<DbRepository>,
        galaxy_version: i32,
    ) -> Self {
        Self {
            rx,
            github,
            repo_service,
            contributor_service,
            galaxy_service,
            db,
            galaxy_version,
        }
    }

    pub async fn run(mut self) {
        while let Some(job) = self.rx.recv().await {
            if let Err(err) = self.process_job(&job).await {
                error!("ingestion job failed: {err}");
            }
        }
    }

    async fn process_job(&self, job: &IngestionJob) -> Result<(), crate::error::AppError> {
        match job {
            IngestionJob::Repo { owner, repo } => self.process_repo(owner, repo).await,
            IngestionJob::SearchWarmup { query } => self.process_search(query).await,
        }
    }

    async fn process_repo(&self, owner: &str, repo: &str) -> Result<(), crate::error::AppError> {
        info!("starting ingestion for {owner}/{repo}");

        // 1) Fetch repository metadata and persist.
        let repo_metadata = self.github.get_repository(owner, repo).await?;
        let stored_repo = self.repo_service.save_repo_from_github(repo_metadata).await?;

        // 2) Fetch contributors, 3) store contributor records, 4) update relationship table.
        let contributors = self.github.get_contributors(owner, repo).await?;
        let saved_contributors = self.contributor_service.save_contributors(&contributors).await?;
        self.contributor_service
            .save_repo_contributor_links(stored_repo.id, &saved_contributors, &contributors)
            .await?;

        // 5) Fetch language distribution, 6) store language stats.
        let languages = self.github.get_languages(owner, repo).await?;
        let lang_rows = languages
            .into_iter()
            .map(|(language, bytes)| RepoLanguageInput { language, bytes })
            .collect::<Vec<_>>();
        self.db.replace_repo_languages(stored_repo.id, &lang_rows).await?;

        // 7) Trigger deterministic galaxy generation and persist cache.
        self.galaxy_service
            .generate_and_cache(&stored_repo, self.galaxy_version)
            .await?;

        info!("ingestion completed for {owner}/{repo}");
        Ok(())
    }

    async fn process_search(&self, query: &str) -> Result<(), crate::error::AppError> {
        info!("starting search warmup for query={query}");
        let hits = self.github.search_repositories(query).await?;

        for item in hits.into_iter().take(10) {
            let repo = self.github.get_repository(&item.owner.login, &item.name).await?;
            self.repo_service.save_repo_from_github(repo).await?;
        }

        info!("search warmup completed for query={query}");
        Ok(())
    }
}
