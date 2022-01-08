use crate::extensions::context::ClientContextExt;

use crate::services::database::guild::Query;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::Message;
use serenity::prelude::Context;

#[command]
pub async fn set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let db = ctx.get_db().await;

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

    let query = Query::from_str(option.to_lowercase().as_str());

    if query.is_none() {
        msg.reply(&ctx.http, "Please provide an valid option")
            .await?;
        return Ok(());
    }

    db.toggle_guild_setting(guild_id, query.unwrap()).await?;

    msg.reply(&ctx.http, format!("Updated {}", option)).await?;

    Ok(())
}
