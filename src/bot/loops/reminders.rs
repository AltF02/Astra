use crate::bot::utils::Utils;
use crate::services::database::get_launch_database;
use crate::services::ConnectionPool;
use serenity::prelude::Context;
use std::error::Error;
use std::sync::Arc;

pub async fn reminder_check(ctx: Arc<Context>) -> Result<(), Box<dyn Error>> {
    let pool = {
        let data = ctx.data.read().await;
        data.get::<ConnectionPool>().unwrap().clone()
    };

    let next_launches = get_launch_database(&pool, true).await;
    for next_launch in &next_launches {
        if next_launch.status != 1 {
            continue;
        }

        let now = chrono::offset::Utc::now();
        let diff = next_launch.net - now;

        let msg = match diff.num_minutes() {
            10 => "10 Minutes until launch!",
            30 => "30 Minutes until launch! (By now the stream will probably be live)",
            60 => "60 Minutes until launch!",
            _ => continue,
        };

        let users = sqlx::query!(
            "SELECT user_id FROM astra.reminders WHERE launch_id = $1",
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
            let user = match Utils::fetch_user_forced(&ctx, user_id).await {
                Some(user) => user,
                None => continue,
            };
            if user
                .dm(&ctx.http, |m| {
                    m.embed(|e| {
                        e.author(|a| a.name(&next_launch.name))
                            .thumbnail(&next_launch.image_url.as_ref().unwrap_or(&"https://launchlibrary1.nyc3.digitaloceanspaces.com/RocketImages/placeholder_1920.png".to_string()))
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
                .await
                .is_err()
            {
                continue;
            }
        }
    }

    Ok(())
}
