//! Updating service for ddnss.de.

use std::process::ExitCode;

use log::{error, info};
use tokio::spawn;

use crate::settings::Settings;
use crate::update::Update;

mod ip_protocol;
mod parse_response;
mod settings;
mod update;

#[tokio::main]
async fn main() -> ExitCode {
    env_logger::init();

    let Ok(config) = Settings::load().inspect_err(|error| error!("Failed to load config: {error}"))
    else {
        return ExitCode::FAILURE;
    };

    let mut exit_code = ExitCode::SUCCESS;

    let tasks: Vec<_> = config
        .into_iter()
        .flat_map(Settings::updates)
        .map(|upd| spawn(update(upd)))
        .collect();

    for task in tasks {
        if let Ok(result) = task.await {
            if result != ExitCode::SUCCESS {
                exit_code = ExitCode::FAILURE;
            }
        } else {
            exit_code = ExitCode::FAILURE;
        }
    }

    exit_code
}

async fn update(update: Update) -> ExitCode {
    match update.run().await {
        Ok(update) => {
            if let Some(amount) = update {
                info!("Updated {amount} hosts.");
            }

            ExitCode::SUCCESS
        }
        Err(error) => {
            error!("Failed to update host: {error}");
            ExitCode::FAILURE
        }
    }
}
