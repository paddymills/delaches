use axum::extract::State;
use axum::http::StatusCode;
use axum::{response::Html, routing::get, Router};
use delaches::member::Member;
use minijinja::{context, Environment};
use std::sync::Arc;

use tower_http::services::ServeDir;

struct AppState {
    env: Environment<'static>,
}

// TODO: custom error type and remove the unrwaps
impl AppState {
    fn new() -> Self {
        let mut env = Environment::new();

        // load templates
        env.add_template("base", include_str!("templates/base.jinja"))
            .unwrap();
        env.add_template("home", include_str!("templates/home.jinja"))
            .unwrap();
        env.add_template("members", include_str!("templates/members.jinja"))
            .unwrap();

        Self { env }
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
        .route("/", get(home))
        .route("/members", get(members))
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    log::info!("Delaches member management app is running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn home(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("home").unwrap();

    let rendered = template
        .render(context! {
            title => "Home",
            welcome_text => "Hello World!",
        })
        .unwrap();

    Ok(Html(rendered))
}

async fn members(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("members").unwrap();

    let rendered = template
        .render(context! {
            title => "Members",
            members => Member::load_data(),
        })
        .unwrap();

    Ok(Html(rendered))
}
