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
        let upserts = contributors
            .iter()
            .map(|c| UpsertContributorInput {
                github_id: c.id,
                username: c.login.clone(),
                avatar_url: c.avatar_url.clone(),
                profile_url: c.html_url.clone(),
            })
            .collect::<Vec<_>>();

        self.db.upsert_contributors_bulk(&upserts).await
    }

    pub async fn save_repo_contributor_links(
        &self,
        repo_id: uuid::Uuid,
        contributor_records: &[ContributorRecord],
        contributor_stats: &[GitHubContributor],
    ) -> AppResult<()> {
        let inputs = contributor_records
            .iter()
            .filter_map(|record| {
                contributor_stats
                    .iter()
                    .find(|c| c.id == record.github_id)
                    .map(|stats| RepoContributorInput {
                        contributor_id: record.id,
                        commits: stats.contributions,
                        additions: 0,
                        deletions: 0,
                        first_commit_at: None,
                        last_commit_at: None,
                    })
            })
            .collect::<Vec<_>>();

        self.db.replace_repo_contributors(repo_id, &inputs).await
    }
}
