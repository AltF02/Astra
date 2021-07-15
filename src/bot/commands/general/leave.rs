use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

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
