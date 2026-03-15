use tokio::sync::mpsc;

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone)]
pub enum IngestionJob {
    Repo { owner: String, repo: String },
    SearchWarmup { query: String },
}

#[derive(Clone)]
pub struct IngestionScheduler {
    tx: mpsc::Sender<IngestionJob>,
}

impl IngestionScheduler {
    pub fn new(tx: mpsc::Sender<IngestionJob>) -> Self {
        Self { tx }
    }

    pub async fn enqueue_repo(&self, owner: &str, repo: &str) -> AppResult<()> {
        let job = IngestionJob::Repo {
            owner: owner.to_string(),
            repo: repo.to_string(),
        };
        self.tx.send(job).await.map_err(|_| AppError::QueueUnavailable)
    }

    pub async fn enqueue_search_warmup(&self, query: &str) -> AppResult<()> {
        let job = IngestionJob::SearchWarmup {
            query: query.to_string(),
        };
        self.tx.send(job).await.map_err(|_| AppError::QueueUnavailable)
    }
}
