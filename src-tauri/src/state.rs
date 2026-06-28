use phoenix_db::connection::DbPool;

pub struct AppState {
    pub pool: DbPool,
}

impl AppState {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}
