use std::sync::Arc;

use chrono::{DateTime, Utc};

use crate::{
    db::repositories::{DbRepository, RepositoryRecord, UpsertRepositoryInput},
    error::AppResult,
    github::{
        client::GitHubClient,
        models::{GitHubRepository, GitHubSearchItem},
    },
};

#[derive(Clone)]
pub struct RepoService {
    db: Arc<DbRepository>,
    github: Arc<GitHubClient>,
}

impl RepoService {
    pub fn new(db: Arc<DbRepository>, github: Arc<GitHubClient>) -> Self {
        Self { db, github }
    }

    pub async fn get_repo(&self, owner: &str, repo: &str) -> AppResult<Option<RepositoryRecord>> {
        self.db.get_repository(owner, repo).await
    }

    pub async fn save_repo_from_github(&self, repo: GitHubRepository) -> AppResult<RepositoryRecord> {
        let input = UpsertRepositoryInput {
            github_id: repo.id,
            owner: repo.owner.login,
            name: repo.name,
            full_name: repo.full_name,
            description: repo.description,
            stars: repo.stars,
            forks: repo.forks,
            language: repo.language,
            created_at: to_utc(repo.created_at),
            updated_at: to_utc(repo.updated_at),
        };
        self.db.upsert_repository(&input).await
    }

    pub async fn search_remote(&self, query: &str) -> AppResult<Vec<GitHubSearchItem>> {
        self.github.search_repositories(query).await
    }

    pub async fn search_local(&self, query: &str) -> AppResult<Vec<RepositoryRecord>> {
        self.db.search_repositories_local(query, 25).await
    }
}

fn to_utc(timestamp: DateTime<Utc>) -> DateTime<Utc> {
    timestamp
}
