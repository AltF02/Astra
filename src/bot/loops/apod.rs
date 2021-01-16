use crate::bot::utils::{check_msg, get_apod, get_channel_forced, Apod};
use crate::services::database::DBGuild;
use crate::services::{Config, ConnectionPool};
use serenity::model::id::ChannelId;
use serenity::prelude::Context;
use std::error::Error;
use std::sync::Arc;

pub async fn send_apod(channel: ChannelId, ctx: &Context, apod: &Apod) {
    check_msg(
        channel
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(&apod.title)
                        .image(&apod.hdurl)
                        .footer(|f| f.text(&apod.date))
                        .description(&apod.explanation)
                        .colour(0x5694c7)
                });
                m
            })
            .await,
    );
}

pub async fn check_apod(ctx: Arc<Context>) -> Result<(), Box<dyn Error>> {
    let ctx_data = ctx.data.read().await;
    let pool = ctx_data.get::<ConnectionPool>().unwrap().clone();
    let config = ctx_data.get::<Config>().unwrap().clone();

    let apod = get_apod(&config.nasa_key).await?;

    let guilds: Vec<DBGuild> = sqlx::query_as!(
        DBGuild,
        "SELECT * FROM astra.guilds WHERE active = true AND apod = true",
    )
    .fetch_all(&pool)
    .await?;

    for guild in guilds {
        if let Some(channel) = get_channel_forced(&ctx, guild.channel_id as u64).await {
            send_apod(channel.id(), &ctx, &apod).await;
        }
    }

    Ok(())
}
