use log::info;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::env;
use std::fs::File;
use std::io::prelude::*;

const ENV_VAR: &'static str = "CONFIG_PATH";
const DEFAULT_LOCATION: &'static str = "./config.yml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub token: String,
    pub prefix: String,
    pub db_uri: String,
    pub nasa_key: String,
}

impl Config {
    pub fn new() -> Config {
        let location = env::var(ENV_VAR).unwrap_or(DEFAULT_LOCATION.to_string());
        match Config::retrieve() {
            Some(conf) => conf,
            None => {
                let conf;
                if prompt("Do you want to configure the config.yml now? (y/n)") == "y" {
                    conf = Config {
                        token: prompt("Discord Bot Token"),
                        prefix: prompt("Bot Prefix"),
                        db_uri: prompt("Database Uri"),
                        nasa_key: prompt("Nasa api key"),
                    }
                } else {
                    conf = Config {
                        token: String::new(),
                        prefix: String::from(";"),
                        db_uri: String::new(),
                        nasa_key: String::new(),
                    };
                }
                conf.save();
                info!("Created a new config.yml to {}", &location);
                return conf;
            }
        }
    }

    pub fn save(&self) {
        let serialized = serde_yaml::to_string(&self).expect("Failed to serialize");
        let location = env::var(ENV_VAR).unwrap_or(DEFAULT_LOCATION.to_string());
        match File::create(&location) {
            Ok(mut file) => file
                .write_all(serialized.as_bytes())
                .expect("Failed to write"),
            Err(e) => panic!("Failed to save config at {}\n{}", &location, e),
        }
    }

    fn retrieve() -> Option<Config> {
        let location = env::var(ENV_VAR).unwrap_or(DEFAULT_LOCATION.to_string());
        match File::open(&location) {
            Ok(mut file) => {
                let mut contents = String::new();
                if let Err(_) = file.read_to_string(&mut contents) {
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
