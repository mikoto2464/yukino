use serde::Serialize;

#[derive(Serialize)]
pub struct Device {
    pub hardware_id: String,
    pub user_id: i64,
    pub name: String,
}