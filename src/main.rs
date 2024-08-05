use axum::extract::State;
use axum::http::StatusCode;
use axum::{response::Html, routing::get, Router};
use delaches::member::Member;
use delaches::user::User;
use minijinja::{context, Environment};
use std::sync::Arc;

use tower_http::services::{ServeDir, ServeFile};

struct AppState {
    fragments: Environment<'static>,
}

// TODO: custom error type and remove the unrwaps
impl AppState {
    fn new() -> Self {
        let mut fragments = Environment::new();

        // load fragment templates
        fragments
            .add_template("members", include_str!("templates/members.jinja"))
            .unwrap();
        fragments
            .add_template("landing", include_str!("templates/landing.jinja"))
            .unwrap();

        Self { fragments }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // build logging for our application
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(std::time::SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        // .level(log::LevelFilter::Error)
        .level_for("delaches", log::LevelFilter::Trace)
        .chain(
            fern::Dispatch::new()
                .level(log::LevelFilter::Debug)
                .chain(std::io::stdout()),
        )
        .chain(
            fern::Dispatch::new().level(log::LevelFilter::Trace).chain(
                std::fs::OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open("server.log")?,
            ),
        )
        .apply()
        .expect("failed to init logging");

    // init state
    let state = Arc::new(AppState::new());

    // build our application with a single route
    let app = Router::new()
        .route_service("/", ServeFile::new("public/index.html"))
        .route_service("/members", ServeFile::new("public/members.html"))
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/style", ServeDir::new("style"))
        .route("/get-members", get(members))
        .route("/landing", get(landing)) // TODO: authentication
        .fallback(fallback)
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    log::info!("Delaches member management app is running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
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

async fn members(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.fragments.get_template("members").unwrap();

    let rendered = template
        .render(context! {
            members => Member::load_data(),
        })
        .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));

    Ok(Html(rendered))
}

async fn fallback() -> (StatusCode, Html<&'static str>) {
    (
        StatusCode::NOT_FOUND,
        Html(include_str!("../public/404.html")),
    )
}
