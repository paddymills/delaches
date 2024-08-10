use axum::http::StatusCode;
use axum::middleware;
use axum::response::{Html, IntoResponse, Redirect};
use axum::routing::{get, post};
use axum::Router;
use dotenv::dotenv;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::services::{ServeDir, ServeFile};

use crate::api::Member;
use crate::auth::Auth;

#[derive(Debug)]
pub struct AppState {
    pub db: SqlitePool,
    pub tokens: Mutex<Vec<String>>,
    pub auth_code: String,
}

impl AppState {
    async fn new(auth_code: String) -> Result<Self, crate::AppError> {
        // init env
        dotenv().unwrap();

        // init database
        let db = SqlitePool::connect(&dotenv::var("DATABASE_URL").unwrap()).await?;

        Ok(Self {
            db,
            tokens: Mutex::new(Vec::new()),
            auth_code,
        })
    }
}

#[derive(Debug)]
pub struct AppServer {}

impl AppServer {
    pub async fn serve(config: crate::Config) -> Result<(), crate::AppError> {
        // TODO: make private key
        let my_key: &[u8] = &[0; 64]; // Your real key must be cryptographically random
        crate::auth::KEY.set(tower_cookies::Key::from(my_key)).ok();

        // init state
        let state = Arc::new(AppState::new(config.code).await?);

        // authenticated routes
        let with_auth = Router::new()
            .route_service("/", ServeFile::new("public/index.html"))
            .nest("/members", Member::routes())
            .route(
                "/auth",
                post(|| async { Redirect::to("/").into_response() }),
            )
            .route_layer(
                middleware::from_extractor_with_state::<Auth, Arc<AppState>>(state.clone()),
            )
            .layer(tower_cookies::CookieManagerLayer::new());

        // unauthenticated routes
        let without_auth = Router::new()
            .route_service("/login", ServeFile::new("public/login.html"))
            .nest_service("/assets", ServeDir::new("assets"))
            .nest_service("/style", ServeDir::new("style"))
            .route("/alive", get(|| async { StatusCode::OK }));

        // build our application with a merged routes
        let app = Router::new()
            .merge(with_auth)
            .merge(without_auth)
            .fallback(fallback)
            .with_state(state);

        // run our app with hyper, listening globally on port 3000
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port)).await?;
        log::info!("server is running at http://localhost:{}", config.port);
        axum::serve(listener, app.into_make_service()).await?;

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
