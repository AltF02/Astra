use crate::bot::utils::Utils;
use crate::extensions::context::ClientContextExt;

use crate::extensions::user::UserExt;
use crate::services::database::reminders::Reminder;
use serenity::prelude::Context;
use std::error::Error;
use std::sync::Arc;

pub async fn reminder_check(ctx: Arc<Context>) -> Result<(), Box<dyn Error>> {
    let db = ctx.get_db().await;

    let next_launches = db.get_limited_launches().await;
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

        let r = Reminder::from(&next_launch.launch_id);
        let users = db.fetch_reminder_users(r).await;

        for user in users {
            let user = match Utils::fetch_user_forced(&ctx, user.0 as u64).await {
                Some(user) => user,
                None => continue,
            };
            let r = user.send_reminder(&ctx, &user, msg, next_launch).await;
            if r.is_err() {
                continue;
            }
        }
    }

    Ok(())
}
