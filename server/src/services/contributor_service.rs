use std::sync::Arc;

use crate::{
    db::repositories::{
        ContributorRecord, DbRepository, RepoContributorInput, UpsertContributorInput,
    },
    error::AppResult,
    github::models::GitHubContributor,
};

#[derive(Clone)]
pub struct ContributorService {
    db: Arc<DbRepository>,
}

impl ContributorService {
    pub fn new(db: Arc<DbRepository>) -> Self {
        Self { db }
    }

    pub async fn save_contributors(
        &self,
        contributors: &[GitHubContributor],
    ) -> AppResult<Vec<ContributorRecord>> {
        let upserts: Vec<UpsertContributorInput> = contributors
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let github_id = c.id.unwrap_or_else(|| -((i as i64) + 1));
                let username = c
                    .login
                    .clone()
                    .unwrap_or_else(|| format!("anonymous-{}", i + 1));
                let profile_url = c
                    .html_url
                    .clone()
                    .unwrap_or_else(|| "https://github.com".to_string());
                UpsertContributorInput {
                    github_id,
                    username,
                    avatar_url: c.avatar_url.clone(),
                    profile_url,
                }
            })
            .collect();

        self.db.upsert_contributors_bulk(&upserts).await
    }

    pub async fn save_repo_contributor_links(
        &self,
        repo_id: uuid::Uuid,
        contributor_records: &[ContributorRecord],
        contributor_stats: &[GitHubContributor],
    ) -> AppResult<()> {
        let inputs: Vec<RepoContributorInput> = contributor_records
            .iter()
            .zip(contributor_stats.iter())
            .map(|(record, stats)| RepoContributorInput {
                contributor_id: record.id,
                commits: stats.contributions,
                additions: 0,
                deletions: 0,
                first_commit_at: None,
                last_commit_at: None,
            })
            .collect();

        self.db.replace_repo_contributors(repo_id, &inputs).await
    }
}
