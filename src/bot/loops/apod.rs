use crate::bot::utils::{Apod, Utils};
use crate::extensions::ClientContextExt;
use crate::services::database::guild::Query;

use serenity::model::id::ChannelId;
use serenity::prelude::Context;
use std::error::Error;
use std::sync::Arc;

pub async fn send_apod(channel: ChannelId, ctx: &Context, apod: &Apod) {
    Utils::check_msg(
        channel
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(&apod.title)
                        .image(&apod.hdurl)
                        .footer(|f| f.text(&apod.date))
                        .description(&apod.explanation)
                        .colour(0x5694c7)
                });
                m
            })
            .await,
    );
}

pub async fn check_apod(ctx: Arc<Context>) -> Result<(), Box<dyn Error>> {
    let (db, config) = ctx.get_db_and_config().await;

    let apod = Apod::fetch(&config.nasa_key).await?;

    let guilds = db.get_guilds_queried(true, Query::Apod).await;

    for guild in guilds {
        if let Some(channel) = Utils::fetch_channel_forced(&ctx, guild.channel_id as u64).await {
            send_apod(channel.id(), &ctx, &apod).await;
        }
    }

    Ok(())
}
