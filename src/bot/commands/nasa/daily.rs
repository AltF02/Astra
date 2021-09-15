use crate::bot::loops::apod::send_apod;
use crate::bot::utils::Apod;
use crate::extensions::context::ClientContextExt;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

#[command]
pub async fn daily(ctx: &Context, msg: &Message) -> CommandResult {
    let config = ctx.get_config().await;
    let channel = msg
        .channel(&ctx)
        .await
        .expect("Unable to fetch message channel?");

    match Apod::fetch(&config.nasa_key).await {
        Ok(body) => {
            send_apod(channel, ctx, &body).await?;
        }
        Err(e) => {
            msg.reply(&ctx, format!("Something went wrong: `{}`", e))
                .await?;
        }
    }

    Ok(())
}
