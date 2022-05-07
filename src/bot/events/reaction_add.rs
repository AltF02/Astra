use crate::bot::utils::Utils;
use crate::extensions::ClientContextExt;
use anyhow::Result;
use log::error;
use serenity::model::prelude::Reaction;
use serenity::prelude::Context;

pub struct ReactionAddEvent;

impl ReactionAddEvent {
    pub async fn run(ctx: &Context, reaction: &Reaction) -> Result<()> {
        if reaction.user_id.unwrap() == ctx.cache.current_user_id()
            || reaction.emoji.to_string() != "🔔"
        {
            return Ok(());
        }

        let message = match ctx.cache.message(reaction.channel_id, reaction.message_id) {
            Some(message) => message,
            None => match reaction.message(&ctx.http).await {
                Ok(message) => message,
                Err(_) => return Ok(()),
            },
        };

        if message.author.id != ctx.cache.current_user_id() {
            return Ok(());
        }

        let embed = &message.embeds[0];
        let id = &embed.footer.as_ref().unwrap().text;
        let name = &embed.title.as_ref().unwrap();

        let user = match ctx.cache.user(reaction.user_id.unwrap()) {
            Some(user) => user,
            None => reaction.user(&ctx.http).await.unwrap(),
        };

        let db = ctx.get_db().await;

        reaction.delete(&ctx.http).await?;
        let db_user =
            sqlx::query("SELECT * FROM astra.reminders WHERE user_id = $1 AND launch_id = $2")
                .bind(user.id.0 as i64)
                .bind(id)
                .fetch_optional(&db.pool)
                .await;

        if let Err(e) = db_user {
            error!("Failed to query, {}", e);
            return Ok(());
        }

        let db_user = db_user.unwrap();
        match db_user {
            Some(_) => {
                Utils::check_msg(
                    user.dm(&ctx.http, |m| {
                        m.embed(|e| {
                            e.title("Reminder Removal")
                                .description(format!(
                                    "I will stop reminding you for launch **{}**",
                                    &name
                                ))
                                // .timestamp(chrono::offset::Utc::now())
                                .colour(0xe6e600_u64)
                        })
                    })
                    .await,
                );
                sqlx::query("DELETE FROM astra.reminders WHERE user_id = $1 AND launch_id = $2")
                    .bind(user.id.0 as i64)
                    .bind(id)
                    .execute(&db.pool)
                    .await?;
            }
            None => {
                Utils::check_msg(user.dm(&ctx.http, |m| { m
                    .embed(|e| { e
                        .title("Reminder Confirmation")
                        .description(format!("I will remind about launch **{}**. If you want to stop me from reminding you, hit the bell emoji again", &name))
                        // .timestamp(chrono::offset::Utc::now())
                        .color(0x15c400_u64)
                    })
                }).await);
                sqlx::query("INSERT INTO astra.reminders (user_id, launch_id) VALUES ($1, $2)")
                    .bind(user.id.0 as i64)
                    .bind(id)
                    .execute(&db.pool)
                    .await?;
            }
        };

        Ok(())
    }
}
