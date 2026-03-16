use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubRepository {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    #[serde(rename = "stargazers_count")]
    pub stars: i32,
    pub forks: i32,
    #[serde(rename = "open_issues_count", default)]
    pub open_issues: i32,
    pub language: Option<String>,
    pub owner: GitHubOwner,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubOwner {
    pub login: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubContributor {
    /// Can be null for anonymous contributors (when anon=1).
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub login: Option<String>,
    pub contributions: i32,
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub html_url: Option<String>,
}

pub type GitHubLanguages = HashMap<String, i64>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubSearchResponse {
    pub total_count: i64,
    pub incomplete_results: bool,
    pub items: Vec<GitHubSearchItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubSearchItem {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub language: Option<String>,
    #[serde(rename = "stargazers_count")]
    pub stars: i32,
    pub forks: i32,
    pub owner: GitHubOwner,
}
