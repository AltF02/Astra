use crate::bot::utils::check_msg;
use crate::bot::utils::{parse_channel, reply};
use crate::services::database::DBGuild;
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

const ENABLED: &str = "<:enabled:796833953256177704>";
const DISABLED: &str = "<:disabled:796833953058521159>";

pub fn format_setting(setting: bool, name: &str) -> String {
    let emote = if setting { ENABLED } else { DISABLED };
    format!("{} **{}**\n", emote, name)
}

#[group()]
#[prefixes("config", "update")]
#[commands(channel, set)]
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
    let guild_db: Option<DBGuild> = sqlx::query_as!(
        DBGuild,
        "SELECT * FROM astra.guilds WHERE guild_id = $1",
        guild_id
    )
    .fetch_optional(&pool)
    .await?;

    match guild_db {
        Some(guild_db) => {
            let mut settings: String = "".to_string();

            settings.push_str(format_setting(guild_db.launches, "Launches").as_str());
            settings.push_str(
                format_setting(guild_db.apod, "APOD (Astronomy Picture of the Day)").as_str(),
            );
            settings.push_str(format_setting(guild_db.events, "Events").as_str());

            check_msg(
                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.title("Guild settings")
                                .description(settings)
                                .footer(|f| {
                                    f.text(&guild.name).icon_url(
                                        &guild.icon_url().unwrap_or_else(|| " ".to_string()),
                                    )
                                })
                                .color(0x00adf8)
                        })
                    })
                    .await,
            );
        }
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
            msg.reply(ctx, "Please provide a channel").await?;
            return Ok(());
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

#[command]
async fn set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let pool = {
        let data = ctx.data.read().await;
        data.get::<ConnectionPool>().unwrap().clone()
    };

    let option = match args.single_quoted::<String>() {
        Ok(arg) => arg,
        Err(_) => {
            msg.reply(
                &ctx.http,
                "Please provide a option to update `>config set [option]",
            )
            .await?;
            return Ok(());
        }
    };

    let guild_id = msg.guild_id.unwrap().0 as i64;

    match option.to_lowercase().as_str() {
        "apod" => {
            sqlx::query!(
                "UPDATE astra.guilds SET apod = NOT apod WHERE guild_id = $1",
                guild_id
            )
            .execute(&pool)
            .await?;
        }
        "launches" => {
            sqlx::query!(
                "UPDATE astra.guilds SET launches = NOT launches WHERE guild_id = $1",
                guild_id
            )
            .execute(&pool)
            .await?;
        }
        "events" => {
            sqlx::query!(
                "UPDATE astra.guilds SET events = NOT events WHERE guild_id = $1",
                guild_id
            )
            .execute(&pool)
            .await?;
        }
        &_ => {
            msg.reply(&ctx.http, "Please provide an valid option")
                .await?;
            return Ok(());
        }
    }

    msg.reply(&ctx.http, format!("Updated {}", option)).await?;

    Ok(())
}
