use crate::bot::loops::apod::send_apod;
use crate::bot::utils::Utils;
use crate::services::Config;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

#[command]
pub async fn daily(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let config = data.get::<Config>().unwrap();

    match Utils::fetch_apod(&config.nasa_key).await {
        Ok(body) => {
            send_apod(msg.channel_id, ctx, &body).await;
        }
        Err(e) => {
            msg.reply(&ctx, format!("Something went wrong: `{}`", e))
                .await?;
        }
    }

    Ok(())
}
