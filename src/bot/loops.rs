use crate::bot::loops::apod::check_apod;
use crate::bot::loops::launches::check_future_launch;
use crate::bot::loops::reminders::reminder_check;
use chrono::Duration;
use log::debug;
use log::error;
use serenity::prelude::Context;
use std::sync::Arc;

pub mod apod;
pub mod launches;
pub mod reminders;
pub mod utils;

pub async fn launches_loop(ctx: Arc<Context>) {
    let launch_ctx = Arc::clone(&ctx);
    let reminder_ctx = Arc::clone(&ctx);
    let apod_ctx = Arc::clone(&ctx);

    tokio::spawn(async move {
        loop {
            debug!("Launches loop started");

            let ctx1 = Arc::clone(&launch_ctx);
            tokio::spawn(async move {
                if let Err(e) = check_future_launch(Arc::clone(&ctx1)).await {
                    error!(
                        "An error occurred while running check_future_launch() >>> {}",
                        e
                    );
                }
            });

            debug!("Launches loop finished");

            tokio::time::delay_for(Duration::minutes(15).to_std().unwrap()).await;
        }
    });

    tokio::spawn(async move {
        loop {
            debug!("Reminder loop started");

            let ctx1 = Arc::clone(&reminder_ctx);
            tokio::spawn(async move {
                if let Err(e) = reminder_check(Arc::clone(&ctx1)).await {
                    error!("reminder_check :: {}", e);
                    eprintln!("An error occurred while running reminder_check() >>> {}", e);
                }
            });

            debug!("Reminder loop stopped");
            tokio::time::delay_for(std::time::Duration::from_secs(60)).await;
        }
    });

    tokio::spawn(async move {
        loop {
            debug!("Apod loop started");

            let ctx1 = Arc::clone(&apod_ctx);
            tokio::spawn(async move {
                if let Err(e) = check_apod(Arc::clone(&ctx1)).await {
                    error!("An error occurred while running check_apod() >>> {}", e);
                }
            });

            debug!("Launches loop finished");

            tokio::time::delay_for(Duration::days(1).to_std().unwrap()).await;
        }
    });
}
