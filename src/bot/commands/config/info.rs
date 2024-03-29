use crate::bot::commands::config::send_settings;
use crate::extensions::{context::ClientContextExt, message::MessageExt};
use crate::services::database::guild::DBGuild;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

#[command]
pub async fn config_info(ctx: &Context, msg: &Message) -> CommandResult {
    let (db, config) = ctx.get_db_and_config().await;
    let guild_id = msg.guild_id.unwrap().0 as i64;
    let guild_db: Option<DBGuild> =
        sqlx::query_as("SELECT * FROM astra.guilds WHERE guild_id = $1")
            .bind(guild_id)
            .fetch_optional(&db.pool)
            .await?;

    match guild_db {
        Some(guild_db) => send_settings(&guild_db, msg, ctx).await?,
        None => {
            msg.reply_error(
                ctx,
                format!(
                    "Guild not configured please run `{}config channel #channel`",
                    config.prefix
                ),
            )
            .await?;
        }
    };
    Ok(())
}
