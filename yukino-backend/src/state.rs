use sqlx::sqlite::SqlitePool;

#[derive(Clone)]
pub struct YukinoState {
    pub tg_secret_key: [u8; 32],
    pub db: SqlitePool,
    pub create_project_key: String,
}
