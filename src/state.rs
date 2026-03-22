use sqlx::sqlite::SqlitePool;

#[derive(Clone)]
pub struct YukinoState {
    pub db: SqlitePool
}