use phoenix_db::connection::DbPool;

pub struct AppState {
    pub pool: DbPool,
    pub vault_key: [u8; 32],
}

impl AppState {
    pub fn new(pool: DbPool, vault_key: [u8; 32]) -> Self {
        Self { pool, vault_key }
    }
}
