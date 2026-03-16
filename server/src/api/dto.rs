use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    db::repositories::RepositoryRecord,
    galaxy::layout::GalaxyLayout,
    github::models::GitHubSearchItem,
};

#[derive(Debug, Serialize)]
pub struct RepositoryResponse {
    pub owner: String,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub stars: i32,
    pub forks: i32,
    pub open_issues: i32,
    pub language: Option<String>,
    pub last_synced_at: DateTime<Utc>,
}

impl From<RepositoryRecord> for RepositoryResponse {
    fn from(value: RepositoryRecord) -> Self {
        Self {
            owner: value.owner,
            name: value.name,
            full_name: value.full_name,
            description: value.description,
            stars: value.stars,
            forks: value.forks,
            open_issues: value.open_issues,
            language: value.language,
            last_synced_at: value.last_synced_at,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum GalaxyResponse {
    Ready {
        version: i32,
        generated_at: DateTime<Utc>,
        galaxy: GalaxyLayout,
    },
    Loading {
        message: String,
        owner: String,
        repo: String,
    },
}

#[derive(Debug, Default, Deserialize)]
pub struct GalaxyQuery {
    /// When true, skip cache and enqueue re-ingestion so the next request gets fresh GitHub data.
    #[serde(default)]
    pub refresh: bool,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub status: String,
    pub message: Option<String>,
    pub items: Vec<SearchItemResponse>,
}

#[derive(Debug, Serialize)]
pub struct SearchItemResponse {
    pub id: i64,
    pub full_name: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub stars: i32,
    pub forks: i32,
}

impl From<GitHubSearchItem> for SearchItemResponse {
    fn from(value: GitHubSearchItem) -> Self {
        Self {
            id: value.id,
            full_name: value.full_name,
            description: value.description,
            language: value.language,
            stars: value.stars,
            forks: value.forks,
        }
    }
}

impl From<RepositoryRecord> for SearchItemResponse {
    fn from(value: RepositoryRecord) -> Self {
        Self {
            id: value.github_id,
            full_name: value.full_name,
            description: value.description,
            language: value.language,
            stars: value.stars,
            forks: value.forks,
        }
    }
}
