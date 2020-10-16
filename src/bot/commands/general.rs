use crate::bot::utils::reply;
use crate::services::Config;
use regex::internal::Input;
use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
    prelude::*,
};

#[group()]
#[commands(ping, prefix, guilds, leave)]
pub struct Commands;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    reply(&ctx, &msg, &String::from("Pong!")).await;
    Ok(())
}

#[command]
async fn prefix(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let config = data.get::<Config>().unwrap();

    if let Err(why) = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Prefix");
                e.description(format!("My prefix is: `{}`", &config.prefix));
                e.color(0xffa500)
            });
            m
        })
        .await
    {
        println!(
            "Failed to send message in #{} because\n{:?}",
            msg.channel_id, why
        );
    };

    Ok(())
}

#[command]
async fn guilds(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(
        &ctx,
        format!(
            "I've been added to **{}** Guilds",
            ctx.cache.guilds().await.len()
        ),
    )
    .await?;
    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    match msg.guild_id {
        Some(guild) => {
            msg.reply(&ctx.http, "I'm sorry to see you go, goodbye 👋")
                .await?;
            guild.leave(&ctx.http).await?;
        }
        None => {
            msg.reply(&ctx.http, "I'm not in a guild? ¯\\_(ツ)_/¯")
                .await?;
        }
    };
    Ok(())
}
