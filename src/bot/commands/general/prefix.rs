use crate::extensions::ClientContextExt;

use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
async fn prefix(ctx: &Context, msg: &Message) -> CommandResult {
    let config = ctx.get_config().await;

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
