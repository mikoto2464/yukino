use axum_login::{AuthnBackend, UserId};
use std::convert::Infallible;
use sqlx::SqlitePool;
use crate::models::credential::Credentials;
use crate::models::user::User;

#[derive(Clone)]
pub struct Backend {
    db: SqlitePool,
}

impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = Infallible;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let devices = sqlx::query_as!(
            Credentials,
            "SELECT hardware_id, user_id as 'user_id!', name FROM devices WHERE user_id = ?",
            creds.provider_id
        )
        if creds.provider == "telegram" && creds.provider_uid == "123456789" {
            // 找到了对应的用户，允许登录！
            Ok(Some(User {
                id: 1,
                username: "tg_master".into(),
                role: Role::User,
                auth_stamp: "some-uuid-v4-string".into(),
            }))
        } else {
            // 未绑定该第三方账号，拒绝登录
            Ok(None)
        }
    }

    // 会话恢复逻辑（完全不需要改变，依然是用 id 查 users 表）
    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        // SELECT * FROM users WHERE id = $1
        if *user_id == 1 {
            Ok(Some(User {
                id: 1,
                username: "tg_master".into(),
                role: Role::User,
                auth_stamp: "some-uuid-v4-string".into(),
            }))
        } else {
            Ok(None)
        }
    }
}