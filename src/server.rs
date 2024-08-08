use crate::api::Member;
use axum::http::StatusCode;
use axum::{response::Html, routing::get, Router};
use dotenv::dotenv;
use sqlx::SqlitePool;
use std::sync::Arc;

use tower_http::services::{ServeDir, ServeFile};

pub struct AppState {
    pub db: SqlitePool,
}

impl AppState {
    async fn new() -> Result<Self, crate::AppError> {
        // init env
        dotenv().unwrap();

        // init database
        let db = SqlitePool::connect(&dotenv::var("DATABASE_URL").unwrap()).await?;

        Ok(Self { db })
    }
}

#[derive(Debug)]
pub struct AppServer {}

impl AppServer {
    pub async fn serve(port: u32) -> Result<(), crate::AppError> {
        // init state
        let state = Arc::new(AppState::new().await?);

        // TODO: authentication
        // build our application with a single route
        let app = Router::new()
            .route_service("/", ServeFile::new("public/index.html"))
            .nest_service("/assets", ServeDir::new("assets"))
            .nest_service("/style", ServeDir::new("style"))
            .nest("/members", Member::routes())
            .route("/alive", get(|| async { StatusCode::OK }))
            .fallback(fallback)
            .with_state(state);

        // run our app with hyper, listening globally on port 3000
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        log::info!("server is running at http://localhost:{}", port);
        axum::serve(listener, app).await?;

        Ok(())
    }

    pub async fn is_running(port: u32) -> bool {
        log::info!("Checking if server is alive at localhost:{}/alive", port);
        match reqwest::get(format!("http://localhost:{}/alive", port)).await {
            Ok(response) => response.status() == StatusCode::OK,
            _ => false,
        }
    }
}

async fn fallback() -> (StatusCode, Html<&'static str>) {
    (
        StatusCode::NOT_FOUND,
        Html(include_str!("../public/404.html")),
    )
}
