use crate::extensions::ClientContextExt;
use crate::services::Db;
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

    match option.to_lowercase().as_str() {
        "apod" => {
            sqlx::query!(
                "UPDATE astra.guilds SET apod = NOT apod WHERE guild_id = $1",
                guild_id
            )
            .execute(&db.pool)
            .await?;
        }
        "launches" => {
            sqlx::query!(
                "UPDATE astra.guilds SET launches = NOT launches WHERE guild_id = $1",
                guild_id
            )
            .execute(&db.pool)
            .await?;
        }
        "events" => {
            sqlx::query!(
                "UPDATE astra.guilds SET events = NOT events WHERE guild_id = $1",
                guild_id
            )
            .execute(&db.pool)
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
