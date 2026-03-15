use std::time::Duration;

use reqwest::{Client, RequestBuilder, StatusCode, header};
use tracing::warn;

use crate::{
    error::{AppError, AppResult},
    github::models::{
        GitHubContributor, GitHubLanguages, GitHubRepository, GitHubSearchItem, GitHubSearchResponse,
    },
};

const GITHUB_API_BASE: &str = "https://api.github.com";
const MAX_RETRIES: usize = 3;

#[derive(Clone)]
pub struct GitHubClient {
    http: Client,
    token: Option<String>,
}

impl GitHubClient {
    pub fn new(token: Option<String>) -> AppResult<Self> {
        let http = Client::builder()
            .timeout(Duration::from_secs(20))
            .user_agent("github-galaxy-backend")
            .build()?;

        Ok(Self { http, token })
    }

    pub async fn get_repository(&self, owner: &str, repo: &str) -> AppResult<GitHubRepository> {
        let url = format!("{GITHUB_API_BASE}/repos/{owner}/{repo}");
        self.send_json(|| self.authenticated_get(&url)).await
    }

    pub async fn get_contributors(&self, owner: &str, repo: &str) -> AppResult<Vec<GitHubContributor>> {
        let mut page = 1;
        let mut all_contributors = Vec::new();

        loop {
            let url = format!("{GITHUB_API_BASE}/repos/{owner}/{repo}/contributors");
            let page_data: Vec<GitHubContributor> = self
                .send_json(|| {
                    self.authenticated_get(&url)
                        .query(&[("per_page", "100"), ("page", &page.to_string())])
                })
                .await?;

            if page_data.is_empty() {
                break;
            }

            all_contributors.extend(page_data);
            page += 1;
        }

        Ok(all_contributors)
    }

    pub async fn get_languages(&self, owner: &str, repo: &str) -> AppResult<GitHubLanguages> {
        let url = format!("{GITHUB_API_BASE}/repos/{owner}/{repo}/languages");
        self.send_json(|| self.authenticated_get(&url)).await
    }

    pub async fn search_repositories(&self, query: &str) -> AppResult<Vec<GitHubSearchItem>> {
        let url = format!("{GITHUB_API_BASE}/search/repositories");
        let response: GitHubSearchResponse = self
            .send_json(|| {
                self.authenticated_get(&url).query(&[
                    ("q", query),
                    ("sort", "stars"),
                    ("order", "desc"),
                    ("per_page", "25"),
                ])
            })
            .await?;

        Ok(response.items)
    }

    fn authenticated_get(&self, url: &str) -> RequestBuilder {
        let mut request = self.http.get(url).header(header::ACCEPT, "application/vnd.github+json");
        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }
        request
    }

    async fn send_json<T, F>(&self, mut builder: F) -> AppResult<T>
    where
        T: serde::de::DeserializeOwned,
        F: FnMut() -> RequestBuilder,
    {
        let mut backoff = Duration::from_millis(300);

        for attempt in 1..=MAX_RETRIES {
            let response = builder().send().await?;

            if response.status() == StatusCode::FORBIDDEN && self.rate_limited(&response) {
                let reset_at = self.parse_rate_limit_reset(&response).unwrap_or_default();
                self.wait_for_rate_limit(reset_at).await;
                return Err(AppError::GitHubRateLimited { reset_at });
            }

            if response.status().is_success() {
                let json = response.json::<T>().await?;
                return Ok(json);
            }

            if self.retryable_status(response.status()) && attempt < MAX_RETRIES {
                warn!("github request failed with status {}, retry {}", response.status(), attempt);
                tokio::time::sleep(backoff).await;
                backoff *= 2;
                continue;
            }

            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::Other(format!(
                "github request failed with status {} and body {}",
                status, body
            )));
        }

        Err(AppError::Other("github retry budget exhausted".to_string()))
    }

    fn retryable_status(&self, status: StatusCode) -> bool {
        status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error()
    }

    fn rate_limited(&self, response: &reqwest::Response) -> bool {
        response
            .headers()
            .get("x-ratelimit-remaining")
            .and_then(|h| h.to_str().ok())
            .map(|v| v == "0")
            .unwrap_or(false)
    }

    fn parse_rate_limit_reset(&self, response: &reqwest::Response) -> Option<i64> {
        response
            .headers()
            .get("x-ratelimit-reset")
            .and_then(|h| h.to_str().ok())
            .and_then(|value| value.parse::<i64>().ok())
    }

    async fn wait_for_rate_limit(&self, reset_at: i64) {
        let now = chrono::Utc::now().timestamp();
        if reset_at <= now {
            return;
        }

        let wait_seconds = (reset_at - now) as u64;
        warn!("github rate limit hit; sleeping for {} seconds", wait_seconds);
        tokio::time::sleep(Duration::from_secs(wait_seconds.min(60))).await;
    }
}
