mod bot;
mod services;
mod api;

use log::{info, warn, LevelFilter};
// use services::config::Config;
use simple_logger::SimpleLogger;
use chrono::{NaiveDateTime, DateTime, Datelike};
use crate::api::launch::get_next_launch;
use crate::services::Config;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    info!("Starting...");

    let config = Config::new();

    if config.token.is_empty() {
        warn!("Please fill out the config.yml");
        return;
    }

    match get_next_launch().await {
        Ok(res) => {
            println!("{}", res.results[0].rocket.configuration.name);
        },
        Err(e) => {
            eprintln!("{}", e)
        }
    };

    bot::start(config).await;
}
