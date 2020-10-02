use crate::bot::utils::{reply, parse_channel};
use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
    prelude::*,
};
use serenity::model::guild::Role;
use serenity::framework::standard::Args;

#[group()]
#[prefixes("set", "config", "update")]
#[commands(ping)]
#[required_permissions(MANAGE_GUILD)]
pub struct Config;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    reply(&ctx, &msg, &String::from("Pong!")).await;
    Ok(())
}

#[command]
async fn channel(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let channel = match args.single_quoted::<String>() {
        Ok(arg) => match parse_channel(ctx, arg).await {
            Some(r) => r,
            None => {
                reply(ctx, msg, "Unable to locate channel").await;
                return Ok(());
            }
        },
        Err(_e) => {
            msg.reply(ctx, "No channel provided").await;
            return Ok(())
        },
    };
    msg.reply(ctx, format!("{}", channel.id())).await;
    Ok(())
}
