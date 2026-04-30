use chrono::{DateTime, Duration, Months};
use serde::Serialize;
use sqlx::Type;

#[derive(Debug, Clone, Serialize)]
pub struct Subscription {
    pub user_id: i64,
    pub project_id: i64,
    pub end: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Type)]
#[sqlx(type_name = "TEXT")]
pub enum Period {
    Hour,
    Day,
    Month,
    Season,
    Year,
}

impl Period {
    pub fn calculate_from(&self, base_timestamp: i64) -> i64 {
        let base_time = DateTime::from_timestamp(base_timestamp, 0).unwrap();

        let expire_time = match self {
            Period::Hour => base_time + Duration::hours(1),
            Period::Day => base_time + Duration::days(1),
            Period::Month => base_time
                .checked_add_months(Months::new(1))
                .expect("overflow"),
            Period::Season => base_time
                .checked_add_months(Months::new(3))
                .expect("overflow"),
            Period::Year => base_time
                .checked_add_months(Months::new(12))
                .expect("overflow"),
        };
        expire_time.timestamp()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Cdkey {
    pub cdkey: String,
    pub project_id: i64,
    pub period: Period,
}
