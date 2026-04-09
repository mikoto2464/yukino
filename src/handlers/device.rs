use crate::models::device::Device;
use crate::state::YukinoState;
use crate::utils::error::YukinoError;
use crate::utils::response::{YukinoJson, YukinoResponse};
use axum::extract::{Path, State};
use std::sync::Arc;

pub async fn get_devices(
    State(state): State<Arc<YukinoState>>,
    Path(user_id): Path<i64>,
) -> Result<YukinoJson<Vec<Device>>, YukinoError> {
    let devices = sqlx::query_as!(
        Device,
        r#"
        select hardware_id, user_id, name
        from devices
        where user_id = ?
        "#,
        user_id
    )
    .fetch_all(&state.db)
    .await?;

    Ok(YukinoResponse::success(devices))
}
