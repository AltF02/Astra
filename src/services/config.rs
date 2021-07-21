use crate::constants::{DEFAULT_LOCATION, ENV_VAR};
use log::info;
use serde::{Deserialize, Serialize};
use serenity::prelude::TypeMapKey;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub token: String,
    pub application_id: u64,
    pub prefix: String,
    pub db_uri: String,
    pub nasa_key: String,
    pub log_channel_id: u64,
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
        let location = env::var(ENV_VAR).unwrap_or_else(|_| DEFAULT_LOCATION.to_string());
        match Config::retrieve() {
            Some(conf) => conf,
            None => {
                let conf;
                if prompt("Do you want to configure the config.yml now? (y/n)") == "y" {
                    conf = Config {
                        token: prompt("Discord Bot Token"),
                        application_id: 0,
                        prefix: prompt("Bot Prefix"),
                        db_uri: prompt("Database Uri"),
                        nasa_key: prompt("Nasa models key"),
                        log_channel_id: 0,
                        emotes: Emotes {
                            enabled: prompt("Discord enabled emote"),
                            disabled: prompt("Discord disabled emote"),
                            bell: prompt("Discord Bell emote"),
                        },
                    }
                } else {
                    conf = Config {
                        token: String::new(),
                        application_id: 0,
                        prefix: String::from(";"),
                        db_uri: String::new(),
                        nasa_key: String::new(),
                        log_channel_id: 0,
                        emotes: Emotes {
                            enabled: String::new(),
                            disabled: String::new(),
                            bell: String::from("🔔"),
                        },
                    };
                }
                conf.save();
                info!("Created a new config.yml to {}", &location);
                conf
            }
        }
    }

    pub fn save(&self) {
        let serialized = serde_yaml::to_string(&self).expect("Failed to serialize");
        let location = env::var(ENV_VAR).unwrap_or_else(|_| DEFAULT_LOCATION.to_string());
        match File::create(&location) {
            Ok(mut file) => file
                .write_all(serialized.as_bytes())
                .expect("Failed to write"),
            Err(e) => panic!("Failed to save config at {}\n{}", &location, e),
        }
    }

    fn retrieve() -> Option<Config> {
        let location = env::var(ENV_VAR).unwrap_or_else(|_| DEFAULT_LOCATION.to_string());
        match File::open(&location) {
            Ok(mut file) => {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_err() {
                    return None;
                };

                match serde_yaml::from_str(&contents) {
                    Ok(des) => Some(des),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }
}

fn prompt(message: &str) -> String {
    println!("{} ->", message);
    let value = &mut String::new();
    std::io::stdin().read_line(value).unwrap();
    return value.trim().to_string();
}
