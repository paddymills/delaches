mod error;
pub use error::AppError;

pub mod api;
pub mod auth;
pub mod server;
pub mod user;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub port: u32,
    pub code: String,
}
