use crate::api::launch::get_next_launch;
use crate::api::url::VidURL;
use crate::api::BASE_URL;
use crate::bot::utils::{check_msg, convert_time_into_str, get_channel_forced, get_user_forced};
use crate::services::database::get_launch_database;
use crate::services::ConnectionPool;
use log::{debug, error, info};
use serenity::model::prelude::ReactionType::Unicode;
use serenity::prelude::*;
use std::{error::Error, sync::Arc};
use time::Duration;

async fn check_future_launch(ctx: Arc<Context>) -> Result<(), Box<dyn Error>> {
    let pool = {
        let data = ctx.data.read().await;
        data.get::<ConnectionPool>().unwrap().clone()
    };

    println!("Getting api data: {}", BASE_URL);
    let next_launches = get_next_launch().await?;
    println!("Here!");
    for next_launch in &next_launches.results {
        if next_launch.tbdtime {
            continue;
        }

        let mut dispatched: bool = false;

        let launch_db = sqlx::query!(
        "SELECT dispatched, net FROM apollo.launches WHERE launch_id = $1 AND dispatched = true",
            next_launch.id
        )
        .fetch_optional(&pool)
        .await?;

        let launch_stamp = &next_launch.net;

        let now = chrono::offset::Utc::now();

        match launch_db {
            Some(launch) => {
                if next_launch.net != launch.net {
                    sqlx::query!(
                        "UPDATE apollo.launches SET net = $1, dispatched = false WHERE launch_id = $2",
                        next_launch.net, next_launch.id)
                        .execute(&pool)
                        .await?;
                }
            }
            None => {
                println!("{}", next_launch.name);
                if launch_stamp > &now {
                    let dt = next_launch.net;
                    let tm = now - Duration::days(1);

                    let remaining_str = convert_time_into_str(dt - now);
                    if 1 > (dt - tm).num_days() {
                        let guilds =
                            sqlx::query!("SELECT * FROM apollo.guilds WHERE active = true")
                                .fetch_all(&pool)
                                .await?;

                        for guild in guilds {
                            let channel_id = guild.channel_id as u64;
                            let channel = match get_channel_forced(&ctx, channel_id).await {
                                Some(channel) => channel,
                                None => {
                                    continue;
                                }
                            };
                            check_msg(
                                channel.id().send_message(&ctx.http, |m| { m
                                    .embed(|e| { e
                                        .title(&next_launch.name)
                                        .description(format!("> {}", if let Some(mission) = &next_launch.mission {&mission.description} else {"No description found :("}))
                                        .fields(vec![
                                            ("Rocket", format!("➤ Name: **{}**\n➤ Probability of launch: **{}**", &next_launch.rocket.configuration.name, if let None = next_launch.probability {"Unknown".to_string()} else {if next_launch.probability.unwrap() == -1 {"Unknown".to_string()} else {format!("{}%", &next_launch.probability.unwrap())}}), false),
                                            // ("Launch Provider",
                                            //  format!("➤ Name: **{}**\n ➤ Country: **{}**",
                                            //          &next_launch.lsp.name,
                                            //          &next_launch.lsp.country_code),
                                            //  false
                                            // ),
                                        ])
                                        .image(&next_launch.rocket.configuration.image_url.as_ref().unwrap_or(&"https://launchlibrary1.nyc3.digitaloceanspaces.com/RocketImages/placeholder_1920.png".to_string()))
                                        .url(&next_launch.vid_urls.get(0).unwrap_or(&VidURL {
                                            priority: 0,
                                            title: "".to_string(),
                                            description: "".to_string(),
                                            feature_image: "".to_string(),
                                            url: "".to_string(),
                                        }).url)
                                        .colour(0x00adf8)
                                        .footer(|f| {f
                                            .text(format!("{}", &next_launch.id))
                                        })
                                        .author(|a| {a
                                            .name(format!("Time Remaining: {} hours", remaining_str))
                                        })
                                        .timestamp(&dt)
                                    })
                                    .reactions(vec![Unicode("🔔".to_string())])
                                }).await,
                            )
                        }
                        dispatched = true;
                    }
                }
            }
        }
        let vid_url: Option<&String> = match next_launch.vid_urls.get(0) {
            Some(vid_url) => Some(&vid_url.url),
            None => None,
        };
        sqlx::query!(
                "INSERT INTO apollo.launches (launch_id, name, net, tbd, vid_url, image_url, dispatched, status) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) ON CONFLICT (launch_id) DO UPDATE SET net = $3, tbd = $4, vid_url = $5, dispatched = $7, status = $8;",
                next_launch.id, next_launch.name, next_launch.net, next_launch.tbdtime, vid_url, next_launch.rocket.configuration.image_url, dispatched, next_launch.status.id as i32)
            .execute(&pool)
            .await?;
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

    let next_launches = get_launch_database(&pool).await;
    for next_launch in &next_launches {
        if next_launch.tbd {
            continue;
        }

        let now = chrono::offset::Utc::now();
        let diff = next_launch.net - now;

        let msg = match diff.num_minutes() {
            10 => "10 Minutes until launch!",
            30 => "30 Minutes until launch! (By now the stream will probably live)",
            60 => "60 Minutes until launch!",
            _ => continue,
        };

        let users = sqlx::query!(
            "SELECT user_id FROM apollo.reminders WHERE launch_id = $1",
            next_launch.launch_id
        )
        .fetch_all(&pool)
        .await?;

        let mut stream = "I'm unaware of any stream :(".to_string();
        if let Some(vid_url) = &next_launch.vid_url {
            stream = format!("[Stream]({})", &vid_url)
        }

        for user in users {
            let user_id = user.user_id as u64;
            let user = match get_user_forced(&ctx, user_id).await {
                Some(user) => user,
                None => continue,
            };
            check_msg(
                user.dm(&ctx.http, |m| {
                    m.embed(|e| {
                        e.author(|a| {
                            a.name(&next_launch.name)
                                .icon_url(&next_launch.image_url.as_ref().unwrap_or(&"https://launchlibrary1.nyc3.digitaloceanspaces.com/RocketImages/placeholder_1920.png".to_string()))
                        })
                        .title("Launch Reminder")
                        .description(format!("{}\n\n{}", msg, stream))
                        .colour(0xcc0099)
                        // .timestamp(&dt)
                        .footer(|f| {
                            f.text(format!(
                                "This reminder is for launch ID: {}",
                                &next_launch.launch_id
                            ))
                            .icon_url(user.face())
                        })
                    })
                })
                .await,
            )
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
                    error!(
                        "An error occurred while running check_future_launch() >>> {}",
                        e
                    );
                }
            });

            debug!("Launches loop finished");

            tokio::time::delay_for(Duration::minutes(15).to_std().unwrap()).await;
        }
    });

    tokio::spawn(async move {
        loop {
            debug!("Reminder loop started");

            let ctx1 = Arc::clone(&ctx_clone);
            tokio::spawn(async move {
                if let Err(e) = reminder_check(Arc::clone(&ctx1)).await {
                    error!("reminder_check :: {}", e);
                    eprintln!("An error occurred while running reminder_check() >>> {}", e);
                }
            });

            debug!("Reminder loop stopped");
            tokio::time::delay_for(std::time::Duration::from_secs(60)).await;
        }
    });
}
