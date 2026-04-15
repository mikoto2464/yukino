use axum_login::AuthUser;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "PascalCase")]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: i64,
    pub nickname: String,
    pub avatar_url: String,
    pub role: Role,
    pub auth_stamp: String,
}

impl AuthUser for User {
    type Id = i64;
    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.auth_stamp.as_bytes()
    }
}
