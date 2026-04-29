use crate::handlers::auth::AuthSession;
use crate::models::device::Device;
use crate::state::YukinoState;
use crate::utils::error::YukinoError;
use crate::utils::response::{YukinoJson, YukinoResponse};
use axum::Json;
use axum::extract::{Path, State};
use serde::Deserialize;
use std::sync::Arc;

pub async fn get_devices(
    State(state): State<Arc<YukinoState>>,
    auth_session: AuthSession,
) -> Result<YukinoJson<Vec<Device>>, YukinoError> {
    let user = auth_session.user.unwrap();

    let devices = sqlx::query_as!(
        Device,
        r#"
        SELECT hardware_id, user_id, name, last_seen
        FROM devices
        WHERE user_id = ?
        "#,
        user.id
    )
    .fetch_all(&state.db)
    .await?;

    Ok(YukinoResponse::success(devices))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDeviceParams {
    hardware_id: String,
    name: String,
}

pub async fn create_device(
    State(state): State<Arc<YukinoState>>,
    auth_session: AuthSession,
    Json(payload): Json<CreateDeviceParams>,
) -> Result<YukinoJson<Device>, YukinoError> {
    let user = auth_session.user.unwrap();
    
    let mut tx = state.db.begin().await?;

    let max_devices = sqlx::query_scalar!(
        r#"
        SELECT max_devices
        FROM users
        WHERE id = ?
        "#,
        user.id
    )
    .fetch_one(&mut *tx)
    .await?;

    let devices_count = sqlx::query_scalar!(
        r#"
        SELECT count(1)
        FROM devices
        WHERE user_id = ?
        "#,
        user.id
    )
    .fetch_one(&mut *tx)
    .await?;

    if devices_count >= max_devices {
        return Err(YukinoError::InvalidParamentsError(
            "The number of available devices has reached its limit.".to_string(),
        ));
    }

    let device = sqlx::query_as!(
        Device,
        r#"
        INSERT INTO devices (hardware_id, user_id, name)
        VALUES (?, ?, ?)
        RETURNING hardware_id, user_id, name, last_seen
        "#,
        payload.hardware_id,
        user.id,
        payload.name
    )
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(YukinoResponse::success(device))
}

pub async fn delete_device(
    State(state): State<Arc<YukinoState>>,
    auth_session: AuthSession,
    Path(hardware_id): Path<String>,
) -> Result<YukinoJson<String>, YukinoError> {
    let user = auth_session.user.unwrap();

    sqlx::query!(
        r#"
        DELETE
        FROM devices
        WHERE user_id = ? AND hardware_id = ?
        "#,
        user.id,
        hardware_id
    )
    .execute(&state.db)
    .await?;

    Ok(YukinoResponse::success("Deleted.".to_string()))
}
