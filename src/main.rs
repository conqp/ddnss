//! Updating service for ddnss.de.

use std::process::ExitCode;

use clap::Parser;
use log::{error, info};

use crate::args::Args;
use crate::config::load;
use crate::update::Update;

mod args;
mod config;
mod ip_protocol;
mod update;

#[tokio::main]
async fn main() -> ExitCode {
    let args = Args::parse();
    env_logger::init();

    let Ok(config) = load().inspect_err(|error| error!("Failed to load config: {error}")) else {
        return ExitCode::FAILURE;
    };

    let mut exit_code = ExitCode::SUCCESS;

    for entry in config {
        match entry.update(args.ip_protocol.into()).await {
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
