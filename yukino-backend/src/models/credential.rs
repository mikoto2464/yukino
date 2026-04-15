use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum Provider {
    Telegram,
    QQ,
}

#[derive(Debug, Clone)]
pub struct AuthCredential {
    pub id: String,
    pub provider: Provider,
    pub nickname: String,
    pub avatar_url: String,
}

#[derive(Debug, Clone)]
pub struct Credential {
    pub id: String,
    pub provider: Provider,
    pub user_id: i64,
}
