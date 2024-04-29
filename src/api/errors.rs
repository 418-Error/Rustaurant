use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde_json::json;
use tracing::error;

pub struct ApiError{
    pub code: StatusCode,
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
       error!("Error: {}", self.message);
       let body = json!({
              "error": self.message,
        });
        (self.code, Json(body)).into_response()
    }
}