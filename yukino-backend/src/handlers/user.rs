
use crate::handlers::auth::AuthSession;
use crate::models::user::Role;
use crate::state::YukinoState;
use crate::utils::error::YukinoError;
use crate::utils::response::{YukinoJson, YukinoResponse};
use axum::extract::{Path, State};
use std::sync::Arc;
use axum::Json;
use serde::Deserialize;
use crate::models::device::Device;
use crate::models::user::User;

pub async fn me(
    State(state): State<Arc<YukinoState>>,
    auth_session: AuthSession
) -> Result<YukinoJson<User>, YukinoError> {
    let user = auth_session.user.unwrap();

    let user = sqlx::query_as!(
        User,
        r#"
        select id , nickname, avatar_url, role as 'role: Role', auth_stamp
        from users
        where id = ?
        "#,
        user.id
    )
        .fetch_one(&state.db)
        .await?;

    Ok(YukinoResponse::success(user))
}

pub async fn devices(
    State(state): State<Arc<YukinoState>>,
    auth_session: AuthSession
) -> Result<YukinoJson<Vec<Device>>, YukinoError> {
    let user = auth_session.user.unwrap();

    let devices = sqlx::query_as!(
        Device,
        r#"
        select hardware_id, user_id, name, last_seen
        from devices
        where user_id = ?
        "#,
        user.id
    )
        .fetch_all(&state.db)
        .await?;

    Ok(YukinoResponse::success(devices))
}

#[derive(Deserialize)]
pub struct BindDeviceParams {
    hardware_id: String,
    name: String,
}

pub async fn create_device(
    State(state): State<Arc<YukinoState>>,
    auth_session: AuthSession,
    Json(payload): Json<BindDeviceParams>
) -> Result<YukinoJson<Device>, YukinoError> {
    let user = auth_session.user.unwrap();

    let max_devices = sqlx::query_scalar!(
        r#"
        select max_devices
        from users
        where id = ?
        "#,
        user.id
    )
        .fetch_one(&state.db)
        .await?;

    let devices_count = sqlx::query_scalar!(
        r#"
        select count(1)
        from devices
        where user_id = ?
        "#,
        user.id
    )
        .fetch_one(&state.db)
        .await?;

    if devices_count >= max_devices {
        return Err(YukinoError::InvalidParamentsError("The number of available devices has reached its limit.".to_string()));
    }

    let device = sqlx::query_as!(
        Device,
        r#"
        insert into devices (hardware_id, user_id, name)
        values (?, ?, ?)
        returning hardware_id, user_id, name, last_seen
        "#,
        payload.hardware_id, user.id, payload.name
    )
        .fetch_one(&state.db)
        .await?;

    Ok(YukinoResponse::success(device))
}

pub async fn delete_device(
    State(state): State<Arc<YukinoState>>,
    auth_session: AuthSession,
    Path(hardware_id): Path<String>
) -> Result<YukinoJson<String>, YukinoError> {
    let user = auth_session.user.unwrap();

    sqlx::query!(
        r#"
        delete
        from devices
        where user_id = ? and hardware_id = ?
        "#,
        user.id, hardware_id
    )
        .execute(&state.db)
        .await?;

    Ok(YukinoResponse::success("".to_string()))
}

pub async fn logout(
    mut auth_session: AuthSession
) -> Result<YukinoJson<String>, YukinoError> {
    auth_session.logout().await?;
    Ok(YukinoResponse::success("".to_string()))
}