use serde::Serialize;

#[derive(Serialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
}
