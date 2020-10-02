use log::warn;
use serenity::model::channel::{Message, Channel};
use serenity::prelude::*;
use serenity::Result as SerenityResult;
use regex::Regex;

pub(crate) async fn reply<T: std::fmt::Display>(ctx: &Context, msg: &Message, content: T) {
    if let Err(why) = msg.channel_id.say(&ctx, &content).await {
        warn!(
            "Failed to send message in #{} because\n{:?}",
            msg.channel_id, why,
        );
    }
}

pub(crate) fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        warn!("Error sending message: {:?}", why);
        eprintln!("{}", why)
    }
}

/*
pub(crate) async fn reply_embed<T>(ctx: &Context, msg: &Message, embed: T) {
    if let Err(why) = msg.channel_id.send_message(&ctx.http, &embed).await {
        println!("Failed to send message in #{} because\n{:?}",
                 msg.channel_id, why
        );
    }
}
*/

pub(crate) async fn parse_channel(
    ctx: &Context,
    channel_name: String,
) -> Option<Channel> {
    let channel: Channel;
    if let Ok(id) = channel_name.parse::<u64>() {
        let channel = match ctx.http.get_channel(id).await {
            Ok(c) => c,
            Err(_e) => return None,
        };
        Some(channel.to_owned())
    } else if channel_name.starts_with("<#") && channel_name.ends_with(">") {
        let re = Regex::new("[<#>]").unwrap();
        let channel_id = re.replace_all(&channel_name, "").into_owned();

        channel = match ctx.http.get_channel(channel_id.parse::<u64>().unwrap()).await
        {
            Ok(m) => m,
            Err(_e) => return None,
        };

        Some(channel.to_owned())
    } else {
        None
    }
}
