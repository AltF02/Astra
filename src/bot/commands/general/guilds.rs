use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

#[command]
async fn guilds(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(
        &ctx,
        format!("I've been added to **{}** Guilds", ctx.cache.guilds().len()),
    )
    .await?;
    Ok(())
}
