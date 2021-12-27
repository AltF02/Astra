use crate::bot::embeds::create_basic_embed;
use anyhow::{Context, Result};
use serenity::{async_trait, builder::CreateEmbed, client, model::prelude::Message};
use std::fmt::Display;

#[async_trait]
pub trait MessageExt {
    async fn reply_embed<F>(&self, ctx: &client::Context, build: F) -> Result<Message>
    where
        F: FnOnce(&mut CreateEmbed) + Send + Sync;

    async fn reply_success(
        &self,
        ctx: &client::Context,
        s: impl Display + Send + Sync + 'static,
    ) -> Result<Message>;

    async fn reply_error(
        &self,
        ctx: &client::Context,
        s: impl Display + Send + Sync + 'static,
    ) -> Result<Message>;
}

#[async_trait]
impl MessageExt for Message {
    async fn reply_embed<F>(&self, ctx: &client::Context, build: F) -> Result<Message>
    where
        F: FnOnce(&mut CreateEmbed) + Send + Sync,
    {
        let mut e = create_basic_embed();
        build(&mut e);

        self.channel_id
            .send_message(ctx, move |m| {
                m.allowed_mentions(|f| f.replied_user(false));
                m.reference_message(self);
                m.set_embed(e)
            })
            .await
            .context("Failed to send embed")
    }

    async fn reply_success(
        &self,
        ctx: &client::Context,
        s: impl Display + Send + Sync + 'static,
    ) -> Result<Message> {
        self.reply_embed(ctx, |e| {
            e.description(s);
            e.color(0xb8bb26);
        })
        .await
    }

    async fn reply_error(
        &self,
        ctx: &client::Context,
        s: impl Display + Send + Sync + 'static,
    ) -> Result<Message> {
        self.reply_embed(ctx, |e| {
            e.description(s);
            e.color(0xe91714);
        })
        .await
    }
}
