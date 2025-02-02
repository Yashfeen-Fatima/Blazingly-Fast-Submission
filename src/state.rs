use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct AppState {
    pub conn_pool: Pool<Postgres>,
}

pub type AppContext = Arc<Mutex<AppState>>;

impl AppState {
    pub fn init(conn_pool: Pool<Postgres>) -> AppContext {
        Arc::new(Mutex::new(Self { conn_pool }))
    }
}