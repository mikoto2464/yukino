use serde::Serialize;

#[derive(Serialize)]
pub struct Subscription {
    pub user_id: i64,
    pub project_id: i64,
    pub end: i64
}