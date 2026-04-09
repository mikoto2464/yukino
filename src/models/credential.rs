use serde::Serialize;

#[derive(Serialize)]
pub enum Provider {
    Telegram,
    QQ
}

#[derive(Clone)]
pub struct Credentials {
    pub id: String,
    pub provider: String,
    pub user_id: i64,
}