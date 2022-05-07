use crate::extensions::ClientContextExt;
use anyhow::Result;
use log::error;
use serenity::client::Context;
use serenity::model::interactions::InteractionResponseType;
use serenity::model::prelude::message_component::ComponentType;
use serenity::model::prelude::{Interaction, InteractionApplicationCommandCallbackDataFlags};

pub struct InteractionCreateEvent;

impl InteractionCreateEvent {
    pub async fn run(ctx: &Context, interaction: Interaction) -> Result<()> {
        let db = ctx.get_db().await;
        let mc = interaction.message_component();

        if mc.is_none() {
            return Ok(());
        }

        let component = mc.unwrap();

        if component.data.component_type != ComponentType::Button
            || component.message.embeds.is_empty()
        {
            return Ok(());
        }

        let reminder =
            sqlx::query("SELECT * FROM astra.reminders WHERE user_id = $1 AND launch_id = $2")
                .bind(component.user.id.0 as i64)
                .bind(&component.data.custom_id)
                .fetch_optional(&db.pool)
                .await;

        if let Err(e) = reminder {
            error!("Failed to query, {:?}", e);
            return Ok(());
        }

        let reminder_exists = reminder.unwrap().is_some();

        let launch_name = component.message.embeds[0].title.as_ref().unwrap();

        let mut content = if reminder_exists {
            format!("I will stop reminding you for **{}**", launch_name)
        } else {
            format!("I will remind you for **{}**! If you want me to stop from reminding you, hit the button again", launch_name)
        };

        let dm_channel = component.user.create_dm_channel(&ctx).await;

        if dm_channel.is_err() {
            content = String::from("Your DM's are closed, please open them and try again.");
        } else {
            let query = if reminder_exists {
                "DELETE FROM astra.reminders WHERE user_id = $1 AND launch_id = $2"
            } else {
                "INSERT INTO astra.reminders (user_id, launch_id) VALUES ($1, $2)"
            };

            sqlx::query(query)
                .bind(component.user.id.0 as i64)
                .bind(&component.data.custom_id)
                .execute(&db.pool)
                .await?;
        }

        component
            .create_interaction_response(&ctx, |r| {
                r.kind(InteractionResponseType::ChannelMessageWithSource);
                r.interaction_response_data(|d| {
                    d.content(content);
                    d.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                });
                r
            })
            .await?;

        Ok(())
    }
}
