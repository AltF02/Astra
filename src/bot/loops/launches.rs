use crate::bot::utils::Utils;
use crate::constants::PLACEHOLDER;
use crate::models::launch::{get_next_launch, Launch};
use crate::models::url::VidURL;
use crate::services::ConnectionPool;
use chrono::{DateTime, Utc};
use serenity::model::prelude::ReactionType::Unicode;
use serenity::prelude::Context;
use sqlx::{Pool, Postgres};
use std::error::Error;
use std::sync::Arc;

pub async fn dispatch_to_guilds(
    ctx: &Context,
    next_launch: &Launch,
    pool: &Pool<Postgres>,
    dt: DateTime<Utc>,
) -> Result<(), Box<dyn Error>> {
    let guilds = sqlx::query!("SELECT * FROM astra.guilds WHERE active = true AND launches = true")
        .fetch_all(pool)
        .await?;

    let remaining_str = Utils::convert_time_into_str(dt - chrono::offset::Utc::now());
    for guild in guilds {
        let channel_id = guild.channel_id as u64;
        let channel = match Utils::fetch_channel_forced(&ctx, channel_id).await {
            Some(channel) => channel,
            None => {
                continue;
            }
        };

        if channel
            .id()
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(&next_launch.name)
                        .description(format!(
                            "> {}",
                            if let Some(mission) = &next_launch.mission {
                                &mission.description
                            } else {
                                "No description found :("
                            }
                        ))
                        .fields(vec![
                            (
                                "Rocket",
                                format!(
                                    "➤ Name: **{}**\n➤ Total Launches: **{}**",
                                    &next_launch.rocket.configuration.name,
                                    &next_launch.rocket.configuration.total_launch_count
                                ),
                                false,
                            ),
                            (
                                "Launch",
                                format!(
                                    "➤ Status: **{}**\n➤ Probability: **{}**",
                                    &next_launch.status.description,
                                    if next_launch.probability.is_none()
                                        || next_launch.probability.unwrap() == -1
                                    {
                                        "Unknown".to_string()
                                    } else {
                                        format!("{}%", &next_launch.probability.unwrap())
                                    }
                                ),
                                false,
                            ),
                        ])
                        .image(
                            &next_launch
                                .rocket
                                .configuration
                                .image_url
                                .as_ref()
                                .unwrap_or(&PLACEHOLDER.to_string()),
                        )
                        .url(
                            &next_launch
                                .vid_urls
                                .get(0)
                                .unwrap_or(&VidURL {
                                    priority: 0,
                                    title: "".to_string(),
                                    description: "".to_string(),
                                    feature_image: Some("".to_string()),
                                    url: "".to_string(),
                                })
                                .url,
                        )
                        .colour(0x00adf8)
                        .footer(|f| f.text(&next_launch.id.to_string()))
                        .author(|a| a.name(format!("Time Remaining: {} hours", remaining_str)))
                        .timestamp(&dt)
                })
                .reactions(vec![Unicode("🔔".to_string())])
            })
            .await
            .is_err()
        {
            continue;
        }
    }
    Ok(())
}

pub async fn check_future_launch(ctx: Arc<Context>) -> Result<(), Box<dyn Error>> {
    let pool = {
        let data = ctx.data.read().await;
        data.get::<ConnectionPool>().unwrap().clone()
    };

    let next_launches = get_next_launch().await?;
    for next_launch in &next_launches.results {
        let mut dispatched: bool = false;
        let launch_stamp = &next_launch.net;
        let now = chrono::offset::Utc::now();

        let launch_db = sqlx::query!(
            "SELECT dispatched, net FROM astra.launches WHERE launch_id = $1 AND dispatched = true",
            next_launch.id
        )
        .fetch_optional(&pool)
        .await?;

        match launch_db {
            Some(launch) => {
                if next_launch.net != launch.net {
                    sqlx::query!(
                        "UPDATE astra.launches SET net = $1, dispatched = false WHERE launch_id = $2",
                        next_launch.net, next_launch.id)
                        .execute(&pool)
                        .await?;
                }
                return Ok(());
            }
            None => {
                let dt = next_launch.net;
                if 24 >= (dt - now).num_hours() && launch_stamp > &now && next_launch.status.id == 1
                {
                    dispatch_to_guilds(&ctx, &next_launch, &pool, dt).await?;
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
        .execute(&pool)
        .await?;
    }

    Ok(())
}
