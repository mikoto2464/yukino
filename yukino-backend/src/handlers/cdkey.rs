use std::sync::Arc;
use axum::extract::{Path, State};
use axum::Json;
use chrono::Utc;
use serde::Deserialize;
use crate::handlers::auth::AuthSession;
use crate::state::YukinoState;
use crate::utils::error::YukinoError;
use crate::utils::response::{YukinoJson, YukinoResponse};
use crate::models::cdkey::{Cdkey, Period};
use crate::models::subscription::Subscription;
use crate::utils::error::YukinoError::InvalidParamentsError;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateCdkeyParams {
    cdkey: String,
}

pub async fn activate_cdkey(
    State(state): State<Arc<YukinoState>>,
    auth_session: AuthSession,
    Json(payload): Json<ActivateCdkeyParams>
) -> Result<YukinoJson<Vec<Subscription>>, YukinoError> {
    if payload.cdkey.is_empty() {
        return Err(InvalidParamentsError("The CD key is invalid.".to_string()));
    }

    let user_id = auth_session.user.unwrap().id;
    let mut tx = state.db.begin().await?;
    let cdkeys = sqlx::query_as!(
        Cdkey,
        r#"
        SELECT cdkey, project_id, period AS 'period: Period'
        FROM cdkeys
        WHERE cdkey = ?
        "#,
        payload.cdkey
    )
        .fetch_all(&mut *tx)
        .await?;

    if cdkeys.is_empty() {
        return Err(InvalidParamentsError("The CD key is invalid.".to_string()));
    }

    sqlx::query!(
        r#"
        DELETE FROM cdkeys
        WHERE cdkey = ?
        "#,
        payload.cdkey
    )
        .execute(&mut *tx)
        .await?;

    let mut subscriptions = Vec::with_capacity(cdkeys.len());
    let now_ts = Utc::now().timestamp();

    for cdkey in cdkeys {
        let current_subscription = sqlx::query_scalar!(
            r#"
            SELECT end
            FROM subscriptions
            WHERE user_id = ? AND project_id = ?
            "#,
            user_id, cdkey.project_id
        )
            .fetch_optional(&mut *tx)
            .await?;

        let base_ts = match current_subscription {
            Some(end) if end > now_ts => end,
            _ => now_ts,
        };

        let new_end = cdkey.period.calculate_from(base_ts);

        let subscription = sqlx::query_as!(
            Subscription,
            r#"
            INSERT INTO subscriptions (user_id, project_id, end)
            VALUES (?, ?, ?)
            ON CONFLICT(user_id, project_id)
            DO UPDATE SET end = excluded.end
            RETURNING user_id, project_id, end
            "#,
            user_id, cdkey.project_id, new_end
        )
            .fetch_one(&mut *tx)
            .await?;

        subscriptions.push(subscription);
    }

    tx.commit().await?;

    Ok(YukinoResponse::success(subscriptions))
}