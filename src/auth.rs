use crate::models::credential::Credential;
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
    type Credentials = Credential;
    type Error = YukinoError;

    async fn authenticate(
        &self,
        credential: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            select u.id, u.nickname, u.avatar_url, u.role as 'role: Role', u.auth_stamp
            from users u
            inner join credentials c on c.user_id = u.id
            where c.id = ? and c.provider = ?
            "#,
            credential.id,
            credential.provider
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(user)
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            select id, nickname, avatar_url, role as 'role: Role', auth_stamp
            from users
            where id = ?
            "#,
            user_id
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(user)
    }
}
