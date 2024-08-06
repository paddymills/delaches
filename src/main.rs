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

    /// import csv file(s) into the database
    #[arg(short, long, num_args = 0..)]
    load: Option<Vec<PathBuf>>,
}

fn init_logging() -> Result<(), Error> {
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
        .apply()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_logging()?;

    let args = Cli::parse();

    if let Some(files) = args.load {
        if !delaches::server::AppServer::is_running(args.port).await {
            return Err(Error::CsvParsingError(String::from(
                "Server needs to be running for files to be loaded",
            )));
        }

        delaches::csv::load_csv_files(files)?;
    } else {
        delaches::server::AppServer::serve(args.port).await?;
    }

    Ok(())
}
