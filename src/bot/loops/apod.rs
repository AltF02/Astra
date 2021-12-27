use crate::extensions::context::ClientContextExt;
use crate::services::database::guild::Query;

use anyhow::Result;

use crate::extensions::ChannelExt;
use crate::models::apod::Apod;
use crate::services::database::apod::DBApod;
use serenity::prelude::Context;
use std::error::Error;
use std::sync::Arc;

pub async fn check_apod(ctx: Arc<Context>) -> Result<(), Box<dyn Error>> {
    let (db, config) = ctx.get_db_and_config().await;

    let apod = Apod::fetch(&config.nasa_key).await?;
    let mut dbapod = DBApod::from(&apod);
    db.get_apod_dispatched(&mut dbapod).await;

    if dbapod.dispatched {
        return Ok(());
    }

    let guilds = db.get_guilds_queried(true, Query::Apod).await;

    for guild in guilds {
        if let Some(channel) = guild.channel_id.fetch(&ctx).await {
            channel.send_apod(&ctx, &apod).await?;
        }
    }

    dbapod.dispatched = true;
    db.set_apod(&dbapod).await?;

    Ok(())
}
