use crate::extensions::{channel::ChannelExt, context::ClientContextExt, duration::DurationExt};
use crate::models::launch::Launch;

use crate::services::database::guild::Query;
use crate::services::database::launch::DBLaunch;
use crate::services::DB;
use chrono::{DateTime, Utc};

use log::warn;

use serenity::model::channel::ReactionType::Unicode;
use serenity::prelude::Context;
use std::error::Error;
use std::sync::Arc;

pub async fn dispatch_to_guilds(
    ctx: &Context,
    next_launch: &Launch,
    db: &DB,
    dt: DateTime<Utc>,
) -> Result<(), Box<dyn Error>> {
    let guilds = db.get_guilds_queried(true, Query::Launches).await;

    let remaining_str = (dt - chrono::offset::Utc::now()).create_24h();
    for guild in guilds {
        let channel = match guild.channel_id.fetch(ctx).await {
            Some(channel) => channel,
            None => {
                continue;
            }
        };

        if let Ok(m) = channel.send_launch(ctx, next_launch, &remaining_str).await {
            m.react(&ctx, Unicode("🔔".to_string())).await?;
        } else {
            warn!("Failed to send Launch to {}", channel.id());
        }
    }
    Ok(())
}

pub async fn check_future_launch(ctx: Arc<Context>) -> Result<(), Box<dyn Error>> {
    let db = ctx.get_db().await;

    let next_launches = Launch::get_next_launch().await?;
    for next_launch in next_launches.results {
        let mut dispatched = false;
        let launch_stamp = &next_launch.net;
        let now = chrono::offset::Utc::now();

        let launch_db = db.get_launch(&next_launch.id, true).await;

        match launch_db {
            Some(launch) => {
                if next_launch.net != launch.net {
                    db.set_net(&next_launch).await?;
                }
                return Ok(());
            }
            None => {
                let dt = next_launch.net;
                if 24 >= (dt - now).num_hours() && launch_stamp > &now && next_launch.status.id == 1
                {
                    dispatch_to_guilds(&ctx, &next_launch, &db, dt).await?;
                    dispatched = true;
                }
            }
        }

        let mut dblaunch = DBLaunch::from(next_launch);
        dblaunch.dispatched = dispatched;

        db.set_launch(dblaunch).await?;
    }

    Ok(())
}
