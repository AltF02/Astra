use crate::extensions::{ChannelExt, ClientContextExt, DurationExt};
use crate::models::launch::Launch;

use crate::services::database::guild::Query;
use crate::services::DB;
use chrono::{DateTime, Utc};

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

        channel
            .send_launch(ctx, next_launch, &remaining_str)
            .await?;
    }
    Ok(())
}

pub async fn check_future_launch(ctx: Arc<Context>) -> Result<(), Box<dyn Error>> {
    let db = ctx.get_db().await;

    let next_launches = Launch::get_next_launch().await?;
    for next_launch in &next_launches.results {
        let mut dispatched: bool = false;
        let launch_stamp = &next_launch.net;
        let now = chrono::offset::Utc::now();

        let launch_db = db.get_launch(&next_launch.id, true).await;

        match launch_db {
            Some(launch) => {
                if next_launch.net != launch.net {
                    sqlx::query!(
                        "UPDATE astra.launches SET net = $1 WHERE launch_id = $2",
                        next_launch.net,
                        next_launch.id
                    )
                    .execute(&db.pool)
                    .await?;
                }
                return Ok(());
            }
            None => {
                let dt = next_launch.net;
                if 24 >= (dt - now).num_hours() && launch_stamp > &now && next_launch.status.id == 1
                {
                    dispatch_to_guilds(&ctx, next_launch, &db, dt).await?;
                    dispatched = true;
                }
            }
        }

        let vid_url: Option<&String> = match next_launch.vid_urls.get(0) {
            Some(vid_url) => Some(&vid_url.url),
            None => None,
        };
        let desc = next_launch
            .mission
            .as_ref()
            .map(|mission| &mission.description);

        sqlx::query!(
            "INSERT INTO astra.launches (launch_id, name, net, vid_url, \
                image_url, dispatched, status, description) \
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8) \
                    ON CONFLICT (launch_id) DO \
                        UPDATE SET net = $3, vid_url = $4, dispatched = $6, \
                        status = $7, description = $8;",
            next_launch.id,
            next_launch.name,
            next_launch.net,
            vid_url,
            next_launch.rocket.configuration.image_url,
            dispatched,
            next_launch.status.id as i32,
            desc
        )
        .execute(&db.pool)
        .await?;
    }

    Ok(())
}
