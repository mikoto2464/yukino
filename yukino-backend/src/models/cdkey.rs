use serde::Serialize;
use sqlx::Type;

#[derive(Debug, Clone, PartialEq, Serialize, Type)]
#[sqlx(type_name = "TEXT")]
pub enum Period {
    Hour,
    Day,
    Month,
    Season,
    Year,
}

#[derive(Debug, Clone, Serialize)]
pub struct Cdkey {
    pub cdkey: String,
    pub project_id: i64,
    pub period: Period,
}
