use crate::bot::utils::Apod;
use crate::extensions::context::ClientContextExt;
use crate::extensions::ChannelExt;
use crate::services::database::guild::Query;

use anyhow::Result;

use log::warn;
use serenity::model::channel::Channel;
use serenity::prelude::Context;
use std::error::Error;
use std::sync::Arc;

pub async fn send_apod(channel: Channel, ctx: &Context, apod: &Apod) -> Result<()> {
    if channel
        .send_embed(ctx, |e| {
            e.title(&apod.title);
            e.image(&apod.hdurl);
            e.description(&apod.explanation);
            e.color(0x5694c7);
        })
        .await
        .err()
    {
        warn!("Failed to send APOD to {}", channel.id())
    }
    Ok(())
}

pub async fn check_apod(ctx: Arc<Context>) -> Result<(), Box<dyn Error>> {
    let (db, config) = ctx.get_db_and_config().await;

    let apod = Apod::fetch(&config.nasa_key).await?;

    let guilds = db.get_guilds_queried(true, Query::Apod).await;

    for guild in guilds {
        if let Some(channel) = guild.channel_id.fetch(&ctx).await {
            send_apod(channel, &ctx, &apod).await?;
        }
    }

    Ok(())
}
