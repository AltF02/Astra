use crate::extensions::context::ClientContextExt;

use crate::extensions::ChannelExt;
use crate::models::apod::Apod;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

#[command]
pub async fn daily(ctx: &Context, msg: &Message) -> CommandResult {
    let db = ctx.get_db().await;
    let channel = msg
        .channel(&ctx)
        .await
        .expect("Unable to fetch message channel?");

    let apod = Apod::from(db.get_most_recent_apod().await);
    channel.send_apod(ctx, &apod).await?;

    Ok(())
}
