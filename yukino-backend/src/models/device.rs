use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Device {
    pub hardware_id: String,
    pub user_id: i64,
    pub name: String,
    pub last_seen: i64,
}
