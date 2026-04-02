use crate::error::YukinoError;
use crate::state::YukinoState;
use axum::extract::{Path, State};
use axum::Json;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct Device {
    id: i64,
    name: String,
    hardware_id: String,
}

pub async fn get_devices(
    State(state): State<Arc<YukinoState>>,
    Path(user_id): Path<i64>
) -> Result<Json<Vec<Device>>, YukinoError> {
    let devices = sqlx::query_as!(
        Device,
        "SELECT id as 'id!', name, hardware_id FROM devices WHERE user_id = ?",
        user_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| YukinoError::DatabaseError(e.to_string()))?;

    Ok(Json(devices))
}