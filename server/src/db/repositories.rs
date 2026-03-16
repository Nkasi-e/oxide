use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::error::AppResult;

#[derive(Debug, Clone)]
pub struct DbRepository {
    pool: PgPool,
}

impl DbRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn upsert_repository(
        &self,
        input: &UpsertRepositoryInput,
    ) -> AppResult<RepositoryRecord> {
        let record = sqlx::query_as::<_, RepositoryRecord>(
            r#"
            INSERT INTO repositories (
                github_id, owner, name, full_name, description, stars, forks, open_issues, language, created_at, updated_at, last_synced_at
            )
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,NOW())
            ON CONFLICT (github_id) DO UPDATE SET
                owner = EXCLUDED.owner,
                name = EXCLUDED.name,
                full_name = EXCLUDED.full_name,
                description = EXCLUDED.description,
                stars = EXCLUDED.stars,
                forks = EXCLUDED.forks,
                open_issues = EXCLUDED.open_issues,
                language = EXCLUDED.language,
                updated_at = EXCLUDED.updated_at,
                last_synced_at = NOW()
            RETURNING id, github_id, owner, name, full_name, description, stars, forks, open_issues, language, created_at, updated_at, last_synced_at
            "#,
        )
        .bind(input.github_id)
        .bind(&input.owner)
        .bind(&input.name)
        .bind(&input.full_name)
        .bind(&input.description)
        .bind(input.stars)
        .bind(input.forks)
        .bind(input.open_issues)
        .bind(&input.language)
        .bind(input.created_at)
        .bind(input.updated_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(record)
    }

    pub async fn get_repository(&self, owner: &str, repo: &str) -> AppResult<Option<RepositoryRecord>> {
        let full_name = format!("{owner}/{repo}");
        let record = sqlx::query_as::<_, RepositoryRecord>(
            r#"
            SELECT id, github_id, owner, name, full_name, description, stars, forks, open_issues, language, created_at, updated_at, last_synced_at
            FROM repositories
            WHERE full_name = $1
            "#,
        )
        .bind(full_name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    pub async fn search_repositories_local(
        &self,
        query: &str,
        limit: i64,
    ) -> AppResult<Vec<RepositoryRecord>> {
        let pattern = format!("%{query}%");
        let rows = sqlx::query_as::<_, RepositoryRecord>(
            r#"
            SELECT id, github_id, owner, name, full_name, description, stars, forks, open_issues, language, created_at, updated_at, last_synced_at
            FROM repositories
            WHERE full_name ILIKE $1 OR description ILIKE $1
            ORDER BY stars DESC
            LIMIT $2
            "#,
        )
        .bind(pattern)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn upsert_contributors_bulk(
        &self,
        contributors: &[UpsertContributorInput],
    ) -> AppResult<Vec<ContributorRecord>> {
        let mut tx = self.pool.begin().await?;
        let mut result = Vec::with_capacity(contributors.len());

        for contributor in contributors {
            let row = sqlx::query_as::<_, ContributorRecord>(
                r#"
                INSERT INTO contributors (github_id, username, avatar_url, profile_url, created_at)
                VALUES ($1,$2,$3,$4,NOW())
                ON CONFLICT (github_id) DO UPDATE SET
                    username = EXCLUDED.username,
                    avatar_url = EXCLUDED.avatar_url,
                    profile_url = EXCLUDED.profile_url
                RETURNING id, github_id, username, avatar_url, profile_url, created_at
                "#,
            )
            .bind(contributor.github_id)
            .bind(&contributor.username)
            .bind(&contributor.avatar_url)
            .bind(&contributor.profile_url)
            .fetch_one(&mut *tx)
            .await?;

            result.push(row);
        }

        tx.commit().await?;
        Ok(result)
    }

    pub async fn replace_repo_contributors(
        &self,
        repo_id: Uuid,
        relations: &[RepoContributorInput],
    ) -> AppResult<()> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("DELETE FROM repo_contributors WHERE repo_id = $1")
            .bind(repo_id)
            .execute(&mut *tx)
            .await?;

        self.insert_repo_contributors(&mut tx, repo_id, relations).await?;
        tx.commit().await?;
        Ok(())
    }

    async fn insert_repo_contributors(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        repo_id: Uuid,
        relations: &[RepoContributorInput],
    ) -> AppResult<()> {
        for relation in relations {
            sqlx::query(
                r#"
                INSERT INTO repo_contributors (
                    repo_id, contributor_id, commits, additions, deletions, first_commit_at, last_commit_at
                )
                VALUES ($1,$2,$3,$4,$5,$6,$7)
                "#,
            )
            .bind(repo_id)
            .bind(relation.contributor_id)
            .bind(relation.commits)
            .bind(relation.additions)
            .bind(relation.deletions)
            .bind(relation.first_commit_at)
            .bind(relation.last_commit_at)
            .execute(&mut **tx)
            .await?;
        }

        Ok(())
    }

    pub async fn replace_repo_languages(
        &self,
        repo_id: Uuid,
        languages: &[RepoLanguageInput],
    ) -> AppResult<()> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("DELETE FROM repo_languages WHERE repo_id = $1")
            .bind(repo_id)
            .execute(&mut *tx)
            .await?;

        for language in languages {
            sqlx::query("INSERT INTO repo_languages (repo_id, language, bytes) VALUES ($1,$2,$3)")
                .bind(repo_id)
                .bind(&language.language)
                .bind(language.bytes)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn get_contributors_for_repo(
        &self,
        repo_id: Uuid,
    ) -> AppResult<Vec<ContributorWithStats>> {
        let rows = sqlx::query_as::<_, ContributorWithStats>(
            r#"
            SELECT
                c.id,
                c.github_id,
                c.username,
                c.avatar_url,
                c.profile_url,
                rc.commits,
                rc.additions,
                rc.deletions
            FROM repo_contributors rc
            JOIN contributors c ON c.id = rc.contributor_id
            WHERE rc.repo_id = $1
            ORDER BY rc.commits DESC, c.username ASC
            "#,
        )
        .bind(repo_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn save_galaxy(
        &self,
        repo_id: Uuid,
        version: i32,
        galaxy_json: &Value,
    ) -> AppResult<GalaxyRecord> {
        let record = sqlx::query_as::<_, GalaxyRecord>(
            r#"
            INSERT INTO galaxies (repo_id, galaxy_json, version, generated_at)
            VALUES ($1,$2,$3,NOW())
            ON CONFLICT (repo_id) DO UPDATE SET
                galaxy_json = EXCLUDED.galaxy_json,
                version = EXCLUDED.version,
                generated_at = NOW()
            RETURNING repo_id, galaxy_json, version, generated_at
            "#,
        )
        .bind(repo_id)
        .bind(galaxy_json)
        .bind(version)
        .fetch_one(&self.pool)
        .await?;

        Ok(record)
    }

    pub async fn get_galaxy_by_full_name(
        &self,
        owner: &str,
        repo: &str,
    ) -> AppResult<Option<GalaxyRecord>> {
        let full_name = format!("{owner}/{repo}");
        let record = sqlx::query_as::<_, GalaxyRecord>(
            r#"
            SELECT g.repo_id, g.galaxy_json, g.version, g.generated_at
            FROM galaxies g
            JOIN repositories r ON r.id = g.repo_id
            WHERE r.full_name = $1
            "#,
        )
        .bind(full_name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RepositoryRecord {
    pub id: Uuid,
    pub github_id: i64,
    pub owner: String,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub stars: i32,
    pub forks: i32,
    pub open_issues: i32,
    pub language: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_synced_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ContributorRecord {
    pub id: Uuid,
    pub github_id: i64,
    pub username: String,
    pub avatar_url: Option<String>,
    pub profile_url: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ContributorWithStats {
    pub id: Uuid,
    pub github_id: i64,
    pub username: String,
    pub avatar_url: Option<String>,
    pub profile_url: String,
    pub commits: i32,
    pub additions: i32,
    pub deletions: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GalaxyRecord {
    pub repo_id: Uuid,
    pub galaxy_json: Value,
    pub version: i32,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct UpsertRepositoryInput {
    pub github_id: i64,
    pub owner: String,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub stars: i32,
    pub forks: i32,
    pub open_issues: i32,
    pub language: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct UpsertContributorInput {
    pub github_id: i64,
    pub username: String,
    pub avatar_url: Option<String>,
    pub profile_url: String,
}

#[derive(Debug, Clone)]
pub struct RepoContributorInput {
    pub contributor_id: Uuid,
    pub commits: i32,
    pub additions: i32,
    pub deletions: i32,
    pub first_commit_at: Option<DateTime<Utc>>,
    pub last_commit_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct RepoLanguageInput {
    pub language: String,
    pub bytes: i64,
}
