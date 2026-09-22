pub mod api;
pub mod config;
pub mod logger;
pub mod middleware;
pub mod web_server;

use std::path::PathBuf;

use actix_web::{App, HttpServer, web};
use auth::{AuthEngine, StartAuthError};
use clap::Parser;
use database::{Database, OpenDatabaseError};
use thiserror::Error;
use utoipa::openapi::{ContactBuilder, InfoBuilder};
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    api::{api_docs_json, mount_api},
    config::{ConfigError, open_or_interactive_configure},
    logger::setup_logger,
    web_server::{not_found, serve},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ServerArgs {
    #[arg(short, long, default_value = "./forgathering.toml")]
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

    #[error("Failed to start auth engine: {0}")]
    StartAuth(#[from] StartAuthError),

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

    let auth_engine = AuthEngine::open(config.auth)?;
    let database = Database::open(config.database).await?;

    HttpServer::new(move || {
        let (builder, mut spec) = App::new()
            .app_data(web::Data::new(auth_engine.clone()))
            .app_data(web::Data::new(database.clone()))
            .into_utoipa_app()
            .service(mount_api())
            .split_for_parts();

        spec.info = InfoBuilder::new()
            .title("Forgathering API")
            .contact(Some(
                ContactBuilder::new()
                    .name(Some("FizzyApple12".to_string()))
                    .url(Some("forgather.ing".to_string()))
                    .email(Some("forgathering@fizzyapple12.com".to_string()))
                    .build(),
            ))
            .version(env!("CARGO_PKG_VERSION").to_string())
            .build();

        builder
            .app_data(web::Data::new(spec.clone()))
            .service(api_docs_json)
            .service(SwaggerUi::new("/docs/{_:.*}").url("/openapi.json", spec))
            .service(serve)
            .default_service(web::to(not_found))
    })
    .bind((config.ip, config.port))?
    .run()
    .await?;

    Ok(())
}
