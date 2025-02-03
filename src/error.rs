use axum::{
    http::{Response, StatusCode},
    response::IntoResponse,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid or expired link {0}")]
    InvalidShortCode(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, error_body) = match self {
            Self::InvalidShortCode(_) => (StatusCode::BAD_REQUEST, &self.to_string()),
        };

        Response::builder()
            .header("Content-Type", "application/json")
            .status(status_code)
            .body(error_body.clone().into())
            .unwrap()
    }
}
