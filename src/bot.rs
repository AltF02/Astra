use crate::services::{config::Config, database};

use events::Handler;
use log::warn;
use serenity::prelude::Mutex;
use serenity::{framework::standard::StandardFramework, prelude::TypeMapKey, Client};

// mod commands;
mod events;
mod loops;
mod utils;

impl TypeMapKey for Config {
    type Value = Config;
}

pub async fn start(config: Config) {
    let framework = StandardFramework::new().configure(|c| {
        c.prefix(&config.prefix);
        c.allow_dm(true);
        c.case_insensitivity(true);
        return c;
    });
    // .group(&commands::general::COMMANDS_GROUP)
    // .group(&commands::nasa::NASA_GROUP)
    // .group(&commands::get::GET_GROUP)
    // .group(&commands::config::CONFIG_GROUP);

    let mut client = Client::new(&config.token)
        .framework(framework)
        .event_handler(Handler {
            run_loops: Mutex::new(true),
        })
        .await
        .expect("Failed to create a new client");

    let pool = database::connect(&config.db_uri).await.unwrap();

    {
        let mut data = client.data.write().await;
        data.insert::<Config>(config);
        data.insert::<database::ConnectionPool>(pool);
    }

    if let Err(e) = client.start_autosharded().await {
        warn!("Failed to login, is the token correct?\n{}", e);
    }
}
