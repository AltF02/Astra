use log::warn;
use regex::Regex;
use serenity::model::channel::{Channel, Message};
use serenity::model::user::User;
use serenity::prelude::*;
use serenity::Result as SerenityResult;

pub struct Utils;

impl Utils {
    pub fn check_msg(result: SerenityResult<Message>) {
        if let Err(why) = result {
            warn!("Error sending message: {:?}", why);
        }
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
}
