use std::sync::Arc;
use axum::extract::{Path, State};
use axum::Json;
use serde::Serialize;
use crate::state::YukinoState;
use crate::error::YukinoError;

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
    .await?;

    Ok(Json(devices))
}