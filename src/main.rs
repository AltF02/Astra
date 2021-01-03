#![allow(clippy::suspicious_else_formatting)]

mod api;
mod bot;
mod services;

use log::{info, warn, LevelFilter};
// use services::config::Config;
use crate::services::Config;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    SimpleLogger::new()
        .with_level(LevelFilter::Error)
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
