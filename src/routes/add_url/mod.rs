use axum::{
    extract::State,
    response::{IntoResponse, Json},
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};

use crate::{db::models::urls::Urls, error::AppError, state::AppContext};

pub fn get_add_url_router() -> Router<AppContext> {
    Router::new().route("/add", post(handle_add_url))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payload {
    pub long_url: String,
}

pub async fn handle_add_url(
    State(ctx): State<AppContext>,
    Json(payload): Json<Payload>,
) -> Result<impl IntoResponse, AppError> {
    let ctx = ctx.clone();
    let ctx = ctx.lock().await;

    let Payload { long_url } = payload;

    Urls::new(long_url).add_to_db(&ctx.conn_pool).await.unwrap();

    Ok("hello_world")
}
