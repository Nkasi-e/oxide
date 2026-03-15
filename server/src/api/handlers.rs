use axum::{
    Json,
    extract::{Path, Query, State},
};

use crate::{
    AppState,
    api::dto::{GalaxyResponse, RepositoryResponse, SearchQuery, SearchResponse},
    error::{AppError, AppResult},
};

pub async fn get_galaxy(
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
) -> AppResult<Json<GalaxyResponse>> {
    if let Some(cached) = state.galaxy_service.get_cached(&owner, &repo).await? {
        let layout = state.galaxy_service.parse_layout(&cached)?;
        return Ok(Json(GalaxyResponse::Ready {
            version: cached.version,
            generated_at: cached.generated_at,
            galaxy: layout,
        }));
    }

    state
        .ingestion_scheduler
        .enqueue_repo(&owner, &repo)
        .await
        .map_err(|_| AppError::QueueUnavailable)?;

    Ok(Json(GalaxyResponse::Loading {
        message: "Repository is being ingested and galaxy will be available shortly".to_string(),
        owner,
        repo,
    }))
}

pub async fn get_repository(
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
) -> AppResult<Json<RepositoryResponse>> {
    let repo = state
        .repo_service
        .get_repo(&owner, &repo)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("repository {owner}/{repo} not found in cache")))?;

    Ok(Json(repo.into()))
}

pub async fn search_repositories(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> AppResult<Json<SearchResponse>> {
    if params.q.trim().is_empty() {
        return Err(AppError::InvalidRequest("query parameter 'q' is required".to_string()));
    }

    let cached = state.repo_service.search_local(&params.q).await?;
    if !cached.is_empty() {
        let items = cached.into_iter().map(Into::into).collect();
        return Ok(Json(SearchResponse {
            status: "ready".to_string(),
            message: None,
            items,
        }));
    }

    state
        .ingestion_scheduler
        .enqueue_search_warmup(&params.q)
        .await
        .map_err(|_| AppError::QueueUnavailable)?;

    Ok(Json(SearchResponse {
        status: "loading".to_string(),
        message: Some("Search warmup enqueued. Retry shortly for cached results.".to_string()),
        items: vec![],
    }))
}
