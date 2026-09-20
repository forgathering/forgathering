use std::{
    fs,
    path::{Path, PathBuf},
};

use database::config::DatabaseConfig;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tui_setup_wizard::{SetupWizard, SetupWizardAnswer, SetupWizardStep};

use crate::logger::LoggingConfig;

fn default_port() -> u16 {
    3000
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerConfig {
    pub logging: LoggingConfig,

    pub database: DatabaseConfig,

    #[serde(default = "default_port")]
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            logging: LoggingConfig {
                file_path: None,
                stdio: true,
            },

            database: DatabaseConfig::Sqlite {
                path: PathBuf::from("./database.db"),
            },

            port: 3000,
        }
    }
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to open config file: {0}")]
    IO(#[from] std::io::Error),

    #[error("Failed to parse config file: {0}")]
    ParseError(#[from] toml::de::Error),

    #[error("Failed to serialize config file: {0}")]
    SerializeError(#[from] toml::ser::Error),

    #[error("Interactive config cancelled")]
    Cancelled,
}

pub fn open_or_interactive_configure(path: &Path) -> Result<ServerConfig, ConfigError> {
    let file = match std::fs::read(path) {
        Ok(file) => file,
        Err(_) => {
            interactive_configure(path)?;

            match std::fs::read(path) {
                Ok(file) => file,
                Err(err) => {
                    return Err(ConfigError::IO(err));
                }
            }
        }
    };

    Ok(toml::from_slice(&file)?)
}

pub fn interactive_configure(path: &Path) -> Result<(), ConfigError> {
    let confirmation_output = &format!("Done! Saving to {} and starting", path.display());

    let setup_wizard_steps = &[
        SetupWizardStep::info("Logging").build(),
        SetupWizardStep::enable("Use file logging?")
            .with_id("enable_logging")
            .build(),
        SetupWizardStep::text("Log file location:")
            .with_default_value("./forgathering.log")
            .with_id("log_file_location")
            .apply_using(|config: &mut ServerConfig, file_path| {
                if !file_path.is_empty() {
                    config.logging.file_path = Some(PathBuf::from(file_path.clone()))
                }
            })
            .only_if("enable_logging", |answer| match answer {
                SetupWizardAnswer::Enable(enable_logging) => *enable_logging,
                _ => false,
            })
            .build(),
        SetupWizardStep::warning("No log file path specified, disabled file logging")
            .only_if("enable_logging", |answer| match answer {
                SetupWizardAnswer::Enable(enable_logging) => *enable_logging,
                _ => false,
            })
            .only_if("log_file_location", |answer| match answer {
                SetupWizardAnswer::Text(log_file_location) => log_file_location.is_empty(),
                _ => false,
            })
            .build(),
        SetupWizardStep::enable("Use console logging?")
            .with_default_value(true)
            .apply_using(|config: &mut ServerConfig, enable_logging| {
                config.logging.stdio = *enable_logging;
            })
            .build(),
        SetupWizardStep::info("Database").build(),
        SetupWizardStep::select(
            "Backend",
            &[
                "Postgres (More robust, Requires external setup)",
                "SQLite (Simple, Built in)",
            ],
        )
        .with_id("database_backend")
        .build(),
        SetupWizardStep::text("Postgres connection URL:")
            .apply_using(|config: &mut ServerConfig, url| {
                config.database = DatabaseConfig::Postgres {
                    url: url.to_string(),
                }
            })
            .validate_using(
                |url| !url.is_empty(),
                "You must input a connection URL for Postgres",
            )
            .validate_using(
                |url| url.starts_with("postgresql://"),
                "Connection URL must use the \"postgresql://\" format",
            )
            .only_if("database_backend", |answer| {
                matches!(answer, SetupWizardAnswer::Select(0))
            })
            .build(),
        SetupWizardStep::text("SQLite database location:")
            .with_default_value("./database.db")
            .apply_using(|config: &mut ServerConfig, file_path| {
                config.database = DatabaseConfig::Sqlite {
                    path: PathBuf::from(file_path.clone()),
                }
            })
            .validate_using(
                |file_path| !file_path.is_empty(),
                "You must input a file path for SQLite",
            )
            .only_if("database_backend", |answer| {
                matches!(answer, SetupWizardAnswer::Select(1))
            })
            .build(),
        SetupWizardStep::info("Web Server").build(),
        SetupWizardStep::unsigned_number("Port:")
            .with_default_value(3000)
            .validate_using(|number| *number >= 1, "Ports must be between 1 and 65535")
            .validate_using(
                |number| *number <= 65535,
                "Ports must be between 1 and 65535",
            )
            .apply_using(|config: &mut ServerConfig, port| {
                config.port = *port as u16;
            })
            .build(),
        SetupWizardStep::info(confirmation_output).build(),
    ];

    let setup_wizard = SetupWizard::<ServerConfig>::new(setup_wizard_steps)
        .with_title("Forgathering Server Setup");

    let Ok(final_config) = setup_wizard.run(ServerConfig::default()) else {
        return Err(ConfigError::Cancelled);
    };

    fs::write(path, toml::to_string_pretty(&final_config)?)?;

    Ok(())
}
