//! Updating service for ddnss.de.

use std::process::ExitCode;

use log::{error, info};

use crate::config::load;

mod config;
mod host;
mod ip_protocol;
mod update;

#[tokio::main]
async fn main() -> ExitCode {
    env_logger::init();

    let Ok(config) = load().inspect_err(|error| error!("Failed to load config: {error}")) else {
        return ExitCode::FAILURE;
    };

    let mut exit_code = ExitCode::SUCCESS;

    for entry in config {
        match entry.update().await {
            Ok(update) => {
                if let Some(amount) = update {
                    info!("Updated {amount} hosts.");
                }
            }
            Err(error) => {
                error!("Failed to update host: {error}");
                exit_code = ExitCode::FAILURE;
            }
        }
    }

    exit_code
}
