use crate::bot::utils::Utils;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    Utils::reply(&ctx, &msg, &String::from("Pong!")).await;
    Ok(())
}
