use crate::utils::error::YukinoError;
use crate::state::YukinoState;
use axum::extract::{Path, State};
use serde::Serialize;
use std::sync::Arc;
use crate::utils::response::{YukinoJson, YukinoResponse};

#[derive(Serialize)]
pub struct Device {
    id: i64,
    name: String,
    hardware_id: String,
}

pub async fn get_devices(
    State(state): State<Arc<YukinoState>>,
    Path(user_id): Path<i64>
) -> Result<YukinoJson<Vec<Device>>, YukinoError> {
    let devices = sqlx::query_as!(
        Device,
        "SELECT id as 'id!', name, hardware_id FROM devices WHERE user_id = ?",
        user_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| YukinoError::DatabaseError(e.to_string()))?;

    Ok(YukinoResponse::success(devices))
}