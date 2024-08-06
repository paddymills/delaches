pub mod member;
pub mod server;
pub mod user;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    LoggingError(#[from] log::SetLoggerError),
    #[error("Requested resource not found")]
    NotFound(String),
}

// Tell axum how to convert `AppError` into a response.
impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            self.to_string(),
        )
            .into_response()
    }
}
