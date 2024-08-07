use std::path::PathBuf;

use clap::Parser;

type Error = delaches::AppError;

#[derive(Debug, clap::Parser)]
#[command(version, about)]
/// Delaches member management system server
pub struct Cli {
    /// port to run server on
    #[arg(short, long, default_value_t = 3000)]
    port: u32,

    /// logging name
    #[arg(long, default_value_t = String::from("server"))]
    log_name: String,

    /// create database
    #[arg(long)]
    init_db: bool,

    /// import csv file(s) into the database
    #[arg(short, long, num_args = 0..)]
    load: Option<Vec<PathBuf>>,
}

fn init_logging(name: &str) -> Result<(), Error> {
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
        .chain(
            fern::Dispatch::new()
                .level(log::LevelFilter::Error)
                .level_for("delaches", log::LevelFilter::Debug)
                .chain(std::io::stdout()),
        )
        .chain(
            fern::Dispatch::new().level(log::LevelFilter::Trace).chain(
                std::fs::OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open(format!("logs/{name}.log"))?,
            ),
        )
        .apply()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Cli::parse();
    init_logging(&args.log_name)?;

    let mut run_server = true;
    if args.init_db {
        let db = rusqlite::Connection::open("db.sqlite")?;
        db.execute_batch(include_str!("schema.sql"))?;
        run_server = false;
    }

    if let Some(files) = args.load {
        if !delaches::server::AppServer::is_running(args.port).await {
            return Err(Error::CsvParsingError(String::from(
                "Server needs to be running for files to be loaded",
            )));
        }

        delaches::csv::load_csv_files(files).await?;
        run_server = false;
    }

    if run_server {
        delaches::server::AppServer::serve(args.port).await?;
    }

    Ok(())
}
