use axum::{
    http::{Response, StatusCode},
    response::IntoResponse,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, error_body) = match self {
            _ => (StatusCode::BAD_REQUEST, "hello".to_owned()),
        };

        Response::builder()
            .header("Content-Type", "application/json")
            .status(status_code)
            .body(error_body.into())
            .unwrap()
    }
}
