use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ConfigError(#[from] toml::de::Error),
    #[error(transparent)]
    LoggingError(#[from] log::SetLoggerError),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    SqliteError(#[from] sqlx::sqlite::SqliteError),
    #[error(transparent)]
    CsvError(#[from] csv::Error),
    #[error("Requested resource not found")]
    NotFound(String),
    #[error("Error parsing csv file")]
    CsvParsingError(String),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("Access Denied")]
    AccessDenied,
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::NotFound(s) => (StatusCode::NOT_FOUND, s),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        }
        .into_response()
    }
}

impl Into<StatusCode> for AppError {
    fn into(self) -> StatusCode {
        self.into_response().status()
    }
}
