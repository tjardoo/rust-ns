use sqlx::mysql::MySqlPool;

pub struct AppState {
    pub pool: MySqlPool,
}
