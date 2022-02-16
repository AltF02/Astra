mod guild_create;
mod reaction_add;
mod ready;

use crate::bot::loops::launches_loop;

use crate::bot::events::guild_create::GuildCreateEvent;
use crate::bot::events::reaction_add::ReactionAddEvent;
use crate::bot::events::ready::ReadyEvent;
use log::info;
use serenity::{async_trait, model::prelude::*, prelude::*};
use std::sync::Arc;

#[derive(Debug)]
pub struct Handler {
    pub(crate) run_loops: Mutex<bool>,
}

#[allow(unused_must_use)]
#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, _ctx: Context, _guilds: Vec<GuildId>) {
        info!("Cache is ready...");
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: bool) {
        GuildCreateEvent::run(&ctx, &guild, &is_new).await;
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        ReactionAddEvent::run(&ctx, &reaction).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        ReadyEvent::run(&ctx, &ready).await;

        if *self.run_loops.lock().await {
            *self.run_loops.lock().await = false;

            let ctx = Arc::new(ctx);

            let ctx_clone = Arc::clone(&ctx);

            let launches_loop = tokio::spawn(async move { launches_loop(ctx_clone).await });

            let _ = launches_loop.await;
            *self.run_loops.lock().await = false;
        }
    }
}
