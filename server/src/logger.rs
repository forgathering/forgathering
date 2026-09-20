use std::{path::PathBuf, time::SystemTime};

use serde::{Deserialize, Serialize};

fn default_stdio() -> bool {
    true
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoggingConfig {
    pub file_path: Option<PathBuf>,

    #[serde(default = "default_stdio")]
    pub stdio: bool,
}

#[cfg(all(debug_assertions, not(feature = "debug_log")))]
pub fn setup_logger(config: LoggingConfig) -> Result<(), fern::InitError> {
    let mut logger = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ));
        })
        .level(log::LevelFilter::Info);

    if config.stdio {
        logger = logger.chain(std::io::stdout());
    }

    if let Some(file_path) = config.file_path {
        logger = logger.chain(fern::log_file(file_path)?);
    }

    logger.apply()?;

    Ok(())
}

#[cfg(all(debug_assertions, feature = "debug_log"))]
pub fn setup_logger(config: LoggingConfig) -> Result<(), fern::InitError> {
    let mut logger = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ));
        })
        .level(log::LevelFilter::Debug);

    if config.stdio {
        logger = logger.chain(std::io::stdout());
    }

    if let Some(file_path) = config.file_path {
        logger = logger.chain(fern::log_file(file_path)?);
    }

    logger.apply()?;

    Ok(())
}

#[cfg(not(debug_assertions))]
pub fn setup_logger(config: LoggingConfig) -> Result<(), fern::InitError> {
    let mut logger = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ));
        })
        .level(log::LevelFilter::Info);

    if config.stdio {
        logger = logger.chain(std::io::stdout());
    }

    if let Some(file_path) = config.file_path {
        logger = logger.chain(fern::log_file(file_path)?);
    }

    logger.apply()?;

    Ok(())
}
