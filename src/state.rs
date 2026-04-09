use sqlx::sqlite::SqlitePool;

#[derive(Clone)]
pub struct YukinoState {
    pub tg_bot_token: [u8; 32],
    pub db: SqlitePool,
}
