use crate::bot::utils::{parse_channel, reply};
use crate::services::ConnectionPool;
use serenity::framework::standard::Args;
use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
    prelude::*,
};

#[group()]
#[prefixes("set", "config", "update")]
#[commands(channel)]
#[default_command(config_info)]
#[required_permissions(MANAGE_CHANNELS)]
pub struct Config;

#[command]
async fn config_info(ctx: &Context, msg: &Message) -> CommandResult {
    reply(&ctx, &msg, &String::from("Pong!")).await;
    Ok(())
}

#[command]
async fn channel(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let channel = match args.single_quoted::<String>() {
        Ok(arg) => match parse_channel(ctx, arg).await {
            Some(r) => r,
            None => {
                msg.reply(ctx, "Unable to locate channel").await?;
                return Ok(());
            }
        },
        Err(_e) => {
            let pool = {
                let data = ctx.data.read().await;
                data.get::<ConnectionPool>().unwrap().clone()
            };
            let guild_id = msg.guild_id.unwrap().0 as i64;

            sqlx::query!(
                "UPDATE apollo.guilds SET active = false WHERE guild_id = $1",
                guild_id
            )
            .execute(&pool)
            .await?;

            msg.reply(ctx, "Disabled apollo reminders for this guild")
                .await?;
            return Ok(()); // TODO Add remove channel
        }
    };

    let pool = {
        let data = ctx.data.read().await;
        data.get::<ConnectionPool>().unwrap().clone()
    };

    let guild_id = msg.guild_id.unwrap().0 as i64;
    let channel_id = channel.id().0 as i64;

    sqlx::query!(
    "INSERT INTO apollo.guilds (guild_id, channel_id) VALUES ($1, $2) ON CONFLICT (guild_id) DO UPDATE SET channel_id = $2, active = true",
        guild_id, channel_id)
            .execute(&pool)
            .await?;

    msg.reply(ctx, format!("Set the channel to {}", channel.mention()))
        .await?;
    Ok(())
}
