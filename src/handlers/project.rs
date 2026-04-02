use crate::utils::error::YukinoError;
use crate::state::YukinoState;
use axum::extract::{Path, State};
use serde::Serialize;
use std::sync::Arc;
use crate::utils::response::{YukinoJson, YukinoResponse};

#[derive(Serialize)]
pub struct Project {
    id: i64,
    name: String,
}

pub async fn create_project(
    State(state): State<Arc<YukinoState>>,
    Path(name): Path<String>,
) -> Result<YukinoJson<Project>, YukinoError> {
    let project = sqlx::query_as!(
        Project,
        "INSERT INTO projects (name) VALUES (?) RETURNING id as 'id!', name",
        name
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| YukinoError::DatabaseError(e.to_string()))?;

    Ok(YukinoResponse::success(project))
}