use std::sync::Arc;
use serenity::prelude::Context;
use log::error;
use log::debug;
use chrono::Duration;
use crate::bot::loops::launches::check_future_launch;
use crate::bot::loops::reminders::reminder_check;

mod launches;
mod reminders;


pub async fn launches_loop(ctx: Arc<Context>) {
    let ctx = Arc::clone(&ctx);
    let ctx_clone = Arc::clone(&ctx);

    tokio::spawn(async move {
        loop {
            debug!("Launches loop started");

            let ctx1 = Arc::clone(&ctx);
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

            let ctx1 = Arc::clone(&ctx_clone);
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
}
