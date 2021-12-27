use crate::models::common::ChannelId;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serenity::prelude::TypeMapKey;
use std::env;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub token: String,
    pub application_id: u64,
    pub prefix: String,
    pub db_uri: String,
    pub nasa_key: String,
    pub log_channel_id: ChannelId,
    pub emotes: Emotes,
}

impl TypeMapKey for Config {
    type Value = Arc<Config>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emotes {
    pub enabled: String,
    pub disabled: String,
    pub bell: String,
}

impl Config {
    pub fn new() -> Config {
        match Config::retrieve() {
            Some(conf) => conf,
            None => {
                panic!("Please create a .env file")
            }
        }
    }

    fn retrieve() -> Option<Config> {
        if dotenv().ok().is_some() {
            Some(Config {
                token: env::var("TOKEN").expect("Missing TOKEN"),
                application_id: env::var("APPLICATION_ID")
                    .expect("Missing APPLICATION_ID")
                    .parse::<u64>()
                    .unwrap(),
                prefix: env::var("PREFIX").expect("Missing PREFIX"),
                db_uri: env::var("DB").expect("Missing DB"),
                nasa_key: env::var("NASA_KEY").expect("Missing NASA_KEY"),
                log_channel_id: ChannelId(
                    env::var("LOG_CHANNEL_ID")
                        .expect("Missing LOG_CHANNEL_ID")
                        .parse::<i64>()
                        .unwrap(),
                ),
                emotes: Emotes {
                    enabled: env::var("EMOTES_ENABLED").expect("Missing EMOTES_ENABLED"),
                    disabled: env::var("EMOTES_DISABLED").expect("Missing EMOTES_DISABLED"),
                    bell: env::var("EMOTES_BELL").expect("Missing EMOTES_BELL"),
                },
            })
        } else {
            None
        }
    }
}
