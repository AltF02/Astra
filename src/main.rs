#![allow(clippy::blocks_in_if_conditions)]
#![allow(clippy::ptr_arg)]

mod bot;
mod constants;
mod extensions;
mod models;
mod services;

use crate::services::{Config, Logger};
use log::{info, warn};

#[tokio::main]
async fn main() {
    fern::Dispatch::setup().expect("Unable to setup logger");

    info!("Starting Astra");

    let config = Config::new();

    if config.token.is_empty() {
        warn!("Please fill out the config.yml");
        return;
    }

    bot::start(config).await;
}
