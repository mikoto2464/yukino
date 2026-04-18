use serde::Deserialize;

pub mod error;
pub mod response;
pub mod telegram_hash;

#[derive(Deserialize)]
pub struct Pagination {
    pub page_size: u64,
    pub page: u64,
}