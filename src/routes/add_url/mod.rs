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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub message: &'static str,
    pub short_link: String,
}

pub async fn handle_add_url(
    State(ctx): State<AppContext>,
    Json(payload): Json<Payload>,
) -> Result<impl IntoResponse, AppError> {
    let ctx = ctx.clone();
    let ctx = ctx.lock().await;

    let Payload { long_url } = payload;
    let mut url_instance = Urls::new(long_url);

    url_instance.add_to_db(&ctx.conn_pool).await.unwrap();

    Ok(Json(Response {
        message: "Short link generated successfully!",
        short_link: format!("http://127.0.0.1:8000/{}", url_instance.short_code),
    }))
}
