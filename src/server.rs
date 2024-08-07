use crate::api::Member;
use crate::user::User;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{response::Html, routing::get, Router};
use minijinja::{context, Environment};
use std::sync::Arc;
use tokio::sync::Mutex;

use tower_http::services::{ServeDir, ServeFile};

pub struct AppState {
    pub fragments: Environment<'static>,
    pub db: Arc<Mutex<rusqlite::Connection>>,
}

impl AppState {
    fn new() -> Result<Self, crate::AppError> {
        let mut fragments = Environment::new();

        // load fragment templates
        fragments.add_template("members", include_str!("templates/members.jinja"))?;
        fragments.add_template("member", include_str!("templates/member.jinja"))?;
        fragments.add_template("landing", include_str!("templates/landing.jinja"))?;

        // init database
        let db = Arc::new(Mutex::new(rusqlite::Connection::open("db.sqlite")?));

        Ok(Self { fragments, db })
    }
}

#[derive(Debug)]
pub struct AppServer {}

impl AppServer {
    pub async fn serve(port: u32) -> Result<(), crate::AppError> {
        // init state
        let state = Arc::new(AppState::new()?);

        // TODO: authentication
        // build our application with a single route
        let app = Router::new()
            .route_service("/", ServeFile::new("public/index.html"))
            .route_service("/members", ServeFile::new("public/members.html"))
            .nest_service("/assets", ServeDir::new("assets"))
            .nest_service("/style", ServeDir::new("style"))
            .route("/landing", get(landing))
            .route(
                "/members/list",
                get(Member::get_members).post(Member::add_member),
            )
            .route(
                "/members/:id",
                get(Member::get_member).post(Member::update_member),
            )
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

async fn landing(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.fragments.get_template("landing").unwrap();

    let rendered = template
        .render(context! {
            user => User::Admin
        })
        .unwrap();

    Ok(Html(rendered))
}

async fn fallback() -> (StatusCode, Html<&'static str>) {
    (
        StatusCode::NOT_FOUND,
        Html(include_str!("../public/404.html")),
    )
}
