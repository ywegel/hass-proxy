use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sqlx::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResponseError {
    #[error("A unexpected database error occurred")]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ResponseError {
    pub fn user_faced_message(&self) -> String {
        match self {
            ResponseError::SqlxError(e) => match e {
                Error::RowNotFound => String::from("No data found"),
                _ => e.to_string()
            },
            ResponseError::Other(e) => String::from("An unexpected error occurred"),
        }
    }

    pub fn system_faced_message(&self) -> String {
        match self {
            ResponseError::SqlxError(e) => format!("Database error: {}", e),
            ResponseError::Other(e) => format!("Unexpected error: {}", e),
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            ResponseError::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ResponseError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        tracing::error!("{}", self.system_faced_message());

        let body = self.user_faced_message();
        let status_code = self.status_code();
        (status_code, body).into_response()
    }
}
