use std::sync::Arc;
use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;
use crate::handlers::auth::AuthSession;
use crate::models::project::Project;
use crate::state::YukinoState;
use crate::utils::error::YukinoError;
use crate::utils::Pagination;
use crate::utils::response::{YukinoJson, YukinoResponse};

#[derive(Deserialize)]
pub struct CreateProjectParams {
    pub create_project_key: String,
    pub name: String,
}

pub async fn create_project(
    State(state): State<Arc<YukinoState>>,
    Json(payload): Json<CreateProjectParams>
) -> Result<YukinoJson<Project>, YukinoError> {
    if payload.create_project_key != state.create_project_key {
        return Err(YukinoError::InvalidParamentsError("Create project key is not correct".to_string()));
    }

    let project = sqlx::query_as!(
        Project,
        r#"
        insert into projects (name)
        values (?)
        returning id, name
        "#,
        payload.name
    )
        .fetch_one(&state.db)
        .await?;

    Ok(YukinoResponse::success(project))
}

pub async fn get_projects(
    State(state): State<Arc<YukinoState>>,
    auth_session: AuthSession,
    Query(params): Query<Pagination>
) -> Result<YukinoJson<Vec<Project>>, YukinoError> {
    let user = auth_session.user.unwrap();

    let limit = params.page_size as i64;
    let offset = (params.page_size - 1) as i64;
    let projects = sqlx::query_as!(
        Project,
        r#"
        select id as 'id: i64', name
        from projects p
        inner join subscriptions s on s.project_id = p.id
        where s.user_id = ?
        limit ? offset ?
        "#,
        user.id, limit, offset
    )
        .fetch_all(&state.db)
        .await?;

    Ok(YukinoResponse::success(projects))
}