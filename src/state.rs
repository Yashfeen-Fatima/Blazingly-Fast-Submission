use sqlx::{Pool, Sqlite};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct AppState {
    pub conn_pool: Pool<Sqlite>,
}

pub type AppContext = Arc<Mutex<AppState>>;

impl AppState {
    pub fn init(conn_pool: Pool<Sqlite>) -> AppContext {
        Arc::new(Mutex::new(Self { conn_pool }))
    }
}
