use crate::bot::utils::check_msg;
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
    let pool = {
        let data = ctx.data.read().await;
        data.get::<ConnectionPool>().unwrap().clone()
    };
    let guild_id = msg.guild_id.unwrap().0 as i64;
    let guild = msg.guild(&ctx).await.unwrap();
    let guild_db = sqlx::query!("SELECT * FROM astra.guilds WHERE guild_id = $1", guild_id)
        .fetch_optional(&pool)
        .await?;
    match guild_db {
        Some(_guild_db) => check_msg(
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title("Guild settings")
                            .description(format!(""))
                            .footer(|f| {
                                f.text(&guild.name)
                                    .icon_url(&guild.icon_url().unwrap_or_else(|| " ".to_string()))
                            })
                    })
                })
                .await,
        ),
        None => {
            reply(
                &ctx,
                &msg,
                "Guild not configured please run `>config channel #channel`",
            )
            .await;
        }
    };
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
                "UPDATE astra.guilds SET active = false WHERE guild_id = $1",
                guild_id
            )
            .execute(&pool)
            .await?;

            msg.reply(ctx, "Disabled astra reminders for this guild")
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
    "INSERT INTO astra.guilds (guild_id, channel_id) VALUES ($1, $2) ON CONFLICT (guild_id) DO UPDATE SET channel_id = $2, active = true",
        guild_id, channel_id)
            .execute(&pool)
            .await?;

    msg.reply(ctx, format!("Set the channel to {}", channel.mention()))
        .await?;
    Ok(())
}
