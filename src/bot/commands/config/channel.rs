use crate::bot::utils::Utils;
use crate::extensions::context::ClientContextExt;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::Message;
use serenity::prelude::*;

#[command]
pub async fn channel(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let channel = match args.single_quoted::<String>() {
        Ok(arg) => match Utils::parse_channel(ctx, arg).await {
            Some(r) => r,
            None => {
                msg.reply(ctx, "Unable to locate channel").await?;
                return Ok(());
            }
        },
        Err(_e) => {
            msg.reply(ctx, "Please provide a channel").await?;
            return Ok(());
        }
    };

    let db = ctx.get_db().await;

    let guild_id = msg.guild_id.unwrap().0 as i64;
    let channel_id = channel.id().0 as i64;

    db.set_guild_channel(channel_id, guild_id).await?;

    msg.reply(ctx, format!("Set the channel to {}", channel.mention()))
        .await?;
    Ok(())
}
