#![allow(clippy::blocks_in_if_conditions)]

mod bot;
mod constants;
mod models;
mod services;

use crate::services::Config;
use log::{info, warn, LevelFilter};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    #[cfg(not(debug_assertions))]
    SimpleLogger::new()
        .with_level(LevelFilter::Error)
        .init()
        .unwrap();

    info!("Starting...");

    let config = Config::new();

    if config.token.is_empty() {
        warn!("Please fill out the config.yml");
        return;
    }

    bot::start(config).await;
}
