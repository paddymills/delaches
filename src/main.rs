use clap::Parser;

type Error = delaches::AppError;

#[derive(Debug, clap::Parser)]
#[command(version, about)]
/// Delaches member management system server
pub struct Cli {
    /// port to run server on
    #[arg(short, long, default_value_t = 3000)]
    port: u32,
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
                    .open(format!("logs/server.log"))?, // TODO: daily log file
            ),
        )
        .apply()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Cli::parse();
    init_logging()?;

    delaches::server::AppServer::serve(args.port).await?;

    Ok(())
}
