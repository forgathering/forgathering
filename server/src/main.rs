pub mod api;
pub mod config;
pub mod logger;
pub mod middleware;
pub mod web_server;

use std::path::PathBuf;

use actix_web::{App, HttpServer, web};
use clap::Parser;
use database::{Database, OpenDatabaseError};
use thiserror::Error;

use crate::{
    config::{ConfigError, open_or_interactive_configure},
    logger::setup_logger,
    web_server::serve,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ServerArgs {
    #[arg(short, long, default_value = "./config.toml")]
    config: PathBuf,
}

#[derive(Error, Debug)]
enum ServerError {
    #[error("Failed to parse arguments: {0}")]
    ArgumentParse(#[from] clap::error::Error),

    #[error("Failed to open or create config file: {0}")]
    ConfigFile(#[from] ConfigError),

    #[error("Failed to start logger: {0}")]
    StartLogger(#[from] fern::InitError),

    #[error("Failed to open database: {0}")]
    OpenDatabase(#[from] OpenDatabaseError),

    #[error("Http server error: {0}")]
    HttpServer(#[from] std::io::Error),
}

#[actix_web::main]
async fn main() -> Result<(), ServerError> {
    let args = ServerArgs::try_parse()?;

    let _ = color_eyre::install();

    let config = match open_or_interactive_configure(&args.config) {
        Ok(config) => config,
        Err(err) => match err {
            ConfigError::Cancelled => {
                println!();
                println!("Interactive configuration cancelled");

                return Ok(());
            }
            err => {
                return Err(ServerError::ConfigFile(err));
            }
        },
    };

    setup_logger(config.logging)?;

    let database = Database::open(config.database).await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .service(serve)
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await?;

    Ok(())
}
