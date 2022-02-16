use crate::bot::utils::Utils;
use crate::extensions::ClientContextExt;
use serenity::model::prelude::Guild;
use serenity::prelude::Context;

pub struct GuildCreateEvent;

impl GuildCreateEvent {
    pub async fn run(ctx: &Context, guild: &Guild, is_new: &bool) {
        if !is_new {
            return;
        }

        let config = ctx.get_config().await;

        match guild.system_channel_id {
            Some(channel) => Utils::check_msg(
                channel
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.title("Thanks for adding me!")
                                .description(
                                    "To start you need to setup a launches channel. \
                                    This can be done with `>config channel #launches`. \
                                    I will send launch reminders in that channel",
                                )
                                .footer(|f| {
                                    f.text(&guild.name).icon_url(
                                        &guild.icon_url().unwrap_or_else(|| " ".to_string()),
                                    )
                                })
                        })
                    })
                    .await,
            ),
            None => return,
        }

        let log_channel = config.log_channel_id.fetch(ctx).await.unwrap();

        let owner_name = match Utils::fetch_user_forced(ctx, guild.owner_id.0).await {
            Some(owner) => owner.name,
            None => "Owner not found".to_string(),
        };

        Utils::check_msg(
            log_channel
                .id()
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title("Joined Guild")
                            .description(format!(
                                "➤ Member count: **{}**\n➤ Owner: **{}**",
                                &guild.member_count, owner_name
                            ))
                            .footer(|f| {
                                f.text(&guild.name)
                                    .icon_url(&guild.icon_url().unwrap_or_else(|| " ".to_string()))
                            })
                            .thumbnail(&guild.icon_url().unwrap_or_else(|| " ".to_string()))
                    })
                })
                .await,
        )
    }
}
