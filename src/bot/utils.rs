use crate::constants::APOD_URL;
use log::warn;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serenity::model::channel::{Channel, Message};
use serenity::model::user::User;
use serenity::prelude::*;
use serenity::Result as SerenityResult;
use std::error::Error;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Apod {
    pub date: String,
    pub explanation: String,
    pub hdurl: String,
    pub media_type: String,
    pub url: String,
    pub title: String,
}

#[derive(Debug)]
pub struct ApodError {
    details: String,
}

impl ApodError {
    fn new(msg: &str) -> ApodError {
        ApodError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ApodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ApodError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub struct Utils;

impl Utils {
    pub fn check_msg(result: SerenityResult<Message>) {
        if let Err(why) = result {
            warn!("Error sending message: {:?}", why);
        }
    }

    pub async fn fetch_channel_forced(ctx: &Context, channel_id: u64) -> Option<Channel> {
        return match ctx.cache.channel(channel_id).await {
            Some(channel) => Some(channel),
            None => {
                if let Ok(channel) = ctx.http.get_channel(channel_id).await {
                    Some(channel)
                } else {
                    return None;
                }
            }
        };
    }

    pub async fn fetch_user_forced(ctx: &Context, user_id: u64) -> Option<User> {
        return match ctx.cache.user(user_id).await {
            Some(user) => Some(user),
            None => match ctx.http.get_user(user_id).await {
                Ok(user) => Some(user),
                Err(_) => None,
            },
        };
    }

    pub async fn parse_channel(ctx: &Context, channel_name: String) -> Option<Channel> {
        let channel: Channel;
        if let Ok(id) = channel_name.parse::<u64>() {
            let channel = match ctx.http.get_channel(id).await {
                Ok(c) => c,
                Err(_e) => return None,
            };
            Some(channel)
        } else if channel_name.starts_with("<#") && channel_name.ends_with('>') {
            let re = Regex::new("[<#>]").unwrap();
            let channel_id = re.replace_all(&channel_name, "").into_owned();

            channel = match ctx
                .http
                .get_channel(channel_id.parse::<u64>().unwrap())
                .await
            {
                Ok(m) => m,
                Err(_e) => return None,
            };

            Some(channel)
        } else {
            None
        }
    }

    pub async fn fetch_apod(key: &str) -> Result<Apod, ApodError> {
        let res = match reqwest::get(format!("{}{}", APOD_URL, key)).await {
            Ok(res) => res,
            Err(e) => return Err(ApodError::new(format!("Reqwest error {}", e).as_str())),
        };

        if !res.status().is_success() {
            return Err(ApodError::new(
                format!("Status code incorrect: {}", res.status().as_u16()).as_str(),
            ));
        }

        return match res.json::<Apod>().await {
            Ok(apod) => Ok(apod),
            Err(e) => Err(ApodError::new(
                format!("Reqwest json error: {}", e).as_str(),
            )),
        };
    }
}
