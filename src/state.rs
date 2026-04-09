use sqlx::sqlite::SqlitePool;

#[derive(Clone)]
pub struct YukinoState {
    pub db: SqlitePool,
    pub tg_bot_token: [u8; 32],
}