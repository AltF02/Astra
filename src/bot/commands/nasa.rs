use crate::bot::utils::check_msg;
use crate::services::Config;
use serde::{Deserialize, Serialize};
use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
    prelude::*,
};

#[derive(Serialize, Deserialize, Debug)]
struct Apod {
    copyright: String,
    date: String,
    explanation: String,
    hdurl: String,
    media_type: String,
    url: String,
}

#[group()]
#[commands(daily)]
pub struct Nasa;

#[command]
async fn daily(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let config = data.get::<Config>().unwrap();

    let res = reqwest::get(&*format!(
        "https://api.nasa.gov/planetary/apod?api_key={}",
        config.nasa_key
    ))
    .await?;
    if !res.status().is_success() {
        msg.reply(&ctx, "Something went wrong").await?;
        return Ok(());
    }

    let body = res.json::<Apod>().await?;
    check_msg(
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(&body.copyright)
                        .image(&body.hdurl)
                        .author(|a| a.name(&body.date))
                        .colour(0xffa500)
                });
                m
            })
            .await,
    );
    Ok(())
}
