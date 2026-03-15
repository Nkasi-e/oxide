use std::sync::Arc;

use serde_json::Value;

use crate::{
    db::repositories::{DbRepository, GalaxyRecord, RepositoryRecord},
    error::AppResult,
    galaxy::{generator::GalaxyGenerator, layout::GalaxyLayout},
};

#[derive(Clone)]
pub struct GalaxyService {
    db: Arc<DbRepository>,
}

impl GalaxyService {
    pub fn new(db: Arc<DbRepository>) -> Self {
        Self { db }
    }

    pub async fn get_cached(&self, owner: &str, repo: &str) -> AppResult<Option<GalaxyRecord>> {
        self.db.get_galaxy_by_full_name(owner, repo).await
    }

    pub async fn generate_and_cache(&self, repo: &RepositoryRecord, version: i32) -> AppResult<GalaxyRecord> {
        let contributors = self.db.get_contributors_for_repo(repo.id).await?;
        let galaxy = GalaxyGenerator::generate(repo, &contributors);
        let galaxy_json: Value = serde_json::to_value(galaxy)?;
        self.db.save_galaxy(repo.id, version, &galaxy_json).await
    }

    pub fn parse_layout(&self, record: &GalaxyRecord) -> AppResult<GalaxyLayout> {
        let layout = serde_json::from_value::<GalaxyLayout>(record.galaxy_json.clone())?;
        Ok(layout)
    }
}
