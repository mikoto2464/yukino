use serde::Serialize;

#[derive(Serialize)]
pub enum Provider {
    Telegram,
    QQ
}

#[derive(Clone)]
pub struct Credentials {
    pub provider: String,
    pub provider_id: String,
    pub user_id: String,
}