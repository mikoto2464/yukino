use crate::utils::error::YukinoError;
use crate::state::YukinoState;
use axum::extract::{Path, State};
use std::sync::Arc;
use crate::models::project::Project;
use crate::utils::response::{YukinoJson, YukinoResponse};

pub async fn create_project(
    State(state): State<Arc<YukinoState>>,
    Path(name): Path<String>,
) -> Result<YukinoJson<Project>, YukinoError> {
    let project = sqlx::query_as!(
        Project,
        r#"
        insert into projects (name)
        values (?)
        RETURNING id, name
        "#,
        name
    )
        .fetch_one(&state.db)
        .await?;

    Ok(YukinoResponse::success(project))
}

pub async fn get_project(
    State(state): State<Arc<YukinoState>>,
    Path(name): Path<String>,
) -> Result<YukinoJson<Project>, YukinoError> {
    let project = sqlx::query_as!(
        Project,
        r#"
        select id, name
        from projects
        where name = ?
        "#,
        name
    )
        .fetch_one(&state.db)
        .await?;

    Ok(YukinoResponse::success(project))
}