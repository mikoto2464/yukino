use crate::models::credential::AuthCredential;
use crate::models::user::{Role, User};
use crate::utils::error::YukinoError;
use axum_login::{AuthnBackend, UserId};
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct Backend {
    pub db: SqlitePool,
}

impl AuthnBackend for Backend {
    type User = User;
    type Credentials = AuthCredential;
    type Error = YukinoError;

    async fn authenticate(
        &self,
        credential: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT u.id, u.nickname, u.avatar_url, u.role AS 'role: Role', u.auth_stamp
            FROM users u
            INNER JOIN credentials c ON c.user_id = u.id
            WHERE c.id = ? AND c.provider = ?
            "#,
            credential.id,
            credential.provider
        )
        .fetch_optional(&self.db)
        .await?;

        if user.is_none() {
            let user = sqlx::query_as!(
                User,
                r#"
                INSERT INTO users (nickname, avatar_url)
                VALUES (?, ?)
                RETURNING id, nickname, avatar_url, role AS 'role: Role', auth_stamp
                "#,
                credential.nickname,
                credential.avatar_url
            )
            .fetch_one(&self.db)
            .await?;

            sqlx::query!(
                r#"
                INSERT INTO credentials (id, provider, user_id)
                VALUES (?, ?, ?)
                "#,
                credential.id,
                credential.provider,
                user.id
            )
            .execute(&self.db)
            .await?;
            return Ok(Some(user));
        }

        Ok(user)
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, nickname, avatar_url, role AS 'role: Role', auth_stamp
            FROM users
            WHERE id = ?
            "#,
            user_id
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(user)
    }
}
