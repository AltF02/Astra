use crate::bot::utils::Utils;
use crate::extensions::ClientContextExt;

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

    sqlx::query!(
    "INSERT INTO astra.guilds (guild_id, channel_id) VALUES ($1, $2) ON CONFLICT (guild_id) DO UPDATE SET channel_id = $2, active = true",
        guild_id, channel_id)
        .execute(&db.pool)
        .await?;

    msg.reply(ctx, format!("Set the channel to {}", channel.mention()))
        .await?;
    Ok(())
}
