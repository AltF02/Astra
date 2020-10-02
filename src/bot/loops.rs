use super::api::get_next_launch;
use crate::bot::utils::check_msg;
use crate::services::ConnectionPool;
use chrono::{DateTime, NaiveDateTime, Utc};
use log::{debug, error, info};
use serenity::prelude::*;
use std::{error::Error, sync::Arc};
use time::Duration;
use serenity::model::prelude::ReactionType::Unicode;

async fn check_future_launch(ctx: Arc<Context>) -> Result<(), Box<dyn Error>> {
    let pool = {
        let data = ctx.data.read().await;
        data.get::<ConnectionPool>().unwrap().clone()
    };

    let next_launches = get_next_launch().await?;
    for next_launch in &next_launches.launches {
        if next_launch.tbdtime == 1 {
            continue
        }

        let mission = &next_launch.missions[0];

        let launch = sqlx::query!(
        "SELECT dispatched, timestamp FROM apollo.launches WHERE launch_id = $1 AND dispatched = true",
        next_launch.id
        )
            .fetch_optional(&pool)
            .await?;

        let launch_stamp =
            DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(next_launch.netstamp, 0), Utc);

        let now = chrono::offset::Utc::now();

        if let None = launch {
            if launch_stamp > now { // Cannot do in one line
                let dt =
                    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(next_launch.netstamp, 0), Utc);
                let tm = now - Duration::days(1);
                let guilds = sqlx::query!("SELECT * FROM apollo.guilds WHERE active = true")
                    .fetch_all(&pool)
                    .await?;

                let diff = dt - now;
                if dt > tm {
                    for guild in guilds {
                        let channel_id = guild.channel_id as u64;
                        let channel = {
                            match ctx.cache.channel(channel_id).await {
                                Some(channel) => channel,
                                None => {
                                    if let Ok(channel) = ctx.http.get_channel(channel_id).await {
                                        channel
                                    } else {
                                        continue;
                                    }
                                }
                            }
                        };
                        check_msg(
                            channel.id().send_message(&ctx.http, |m| { m
                                .embed(|e| { e
                                    .title(&next_launch.name)
                                    .description(format!("> {}", &mission.description))
                                    .fields(vec![
                                        ("Rocket", format!("➤ Name: **{}**\n➤ Probability of launch: **{}**", &next_launch.rocket.name, if &next_launch.probability == &-1_i8 {"Unknown".to_string()} else {format!("{}%", &next_launch.probability)}), false),
                                        ("Launch Provider",
                                         format!("➤ Name: **{}**\n ➤ Country: **{}**",
                                                 &next_launch.lsp.name,
                                                 &next_launch.lsp.country_code),
                                         false
                                        ),
                                    ])
                                    .image(&next_launch.rocket.image_url)
                                    .url(&next_launch.vid_urls[0])
                                    .colour(0x00adf8)
                                    .footer(|f| {f
                                        .text(format!("{}", &next_launch.id))
                                    })
                                    .author(|a| {a
                                        .name(format!("Time Remaining: {}:{}", diff.num_hours(), diff.num_minutes() - 60 * diff.num_hours()))
                                    })
                                    .timestamp(&dt)
                                })
                                .reactions(vec![Unicode("🔔".to_string())])
                            }).await,
                        )
                    }

                    sqlx::query!(
                "INSERT INTO apollo.launches (launch_id, dispatched, timestamp) VALUES ($1, true, $2) ON CONFLICT (launch_id) DO UPDATE SET timestamp = $2;",
                next_launch.id, next_launch.netstamp).execute(&pool).await?;
                }
            }
        }
    }

    // let channel = ctx.cache.channel(761357003762827274).await.unwrap();
    // check_msg(channel.id().say(&ctx, &next_launch.name).await);
    Ok(())
}

async fn reminder_check(ctx: Arc<Context>) -> Result<(), Box<dyn Error>> {
    let pool = {
        let data = ctx.data.read().await;
        data.get::<ConnectionPool>().unwrap().clone()
    };

    let next_launches = get_next_launch().await?;
    for next_launch in  &next_launches.launches {
        if next_launch.tbdtime == 1 {
            continue
        }

        let dt =
            DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(next_launch.netstamp, 0), Utc);
        let now = chrono::offset::Utc::now();
        let diff = dt - now;

        let msg = match diff.num_minutes() {
            10 => "10 Minutes until launch!",
            30 => "30 Minutes until launch! (By now the stream will probably live)",
            60 => "60 Minutes until launch!",
            _ => continue
        };

        let users = sqlx::query!("SELECT user_id FROM apollo.reminders WHERE launch_id = $1", next_launch.id)
            .fetch_all(&pool)
            .await?;

        let mut stream = "I'm unaware of any stream :(".to_string();
        let link;
        if !next_launch.vid_urls.is_empty() {
            link = &next_launch.vid_urls[0];
            stream = format!("[Stream]({})", &link)
        }

        for user in users {
            let user_id = user.user_id as u64;
            let user = match ctx.cache.user(user_id).await {
                Some(user) => user,
                None => match ctx.http.get_user(user_id).await {
                    Ok(user) => user,
                    Err(_) => continue
                }
            };
            check_msg(user.dm(&ctx.http, |m| {m
                .embed(|e| {e
                    .author(|a| {a
                        .name(&next_launch.name)
                        .icon_url(&next_launch.rocket.image_url)
                    })
                    .title("Launch Reminder")
                    .description(format!("{}\n\n{}", msg, stream))
                    .colour(0xcc0099)
                    .timestamp(&dt)
                    .footer(|f| {f
                        .text(format!("This reminder is for launch ID: {}", &next_launch.id))
                        .icon_url(user.face())
                    })
                })
            }).await)
        }

    }

    Ok(())
}

pub async fn launches_loop(ctx: Arc<Context>) {
    let ctx = Arc::clone(&ctx);
    let ctx_clone = Arc::clone(&ctx);

    tokio::spawn(async move {
        loop {
            debug!("Launches loop started");

            let ctx1 = Arc::clone(&ctx);
            tokio::spawn(async move {
                if let Err(e) = check_future_launch(Arc::clone(&ctx1)).await {
                    error!("check_future_launch :: {}", e);
                    eprintln!(
                        "An error occurred while running check_future_launch() >>> {}",
                        e
                    );
                }
            });

            debug!("Launches loop finished");

            tokio::time::delay_for(std::time::Duration::from_secs(120)).await;
        }
    });

    tokio::spawn(async move {
        loop {
            debug!("Reminder loop started");

            let ctx1 = Arc::clone(&ctx_clone);
            tokio::spawn(async move {
                if let Err(e) = reminder_check(Arc::clone(&ctx1)).await {
                    error!("reminder_check :: {}", e);
                    eprintln!(
                        "An error occurred while running reminder_check() >>> {}",
                        e
                    );
                }
            });

            debug!("Reminder loop stopped");
            tokio::time::delay_for(std::time::Duration::from_secs(15)).await;
        }
    });
}
