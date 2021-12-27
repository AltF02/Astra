use crate::bot::embeds::{create_apod_embed, create_basic_embed, create_launch_embed};
use crate::models::apod::Apod;
use crate::models::launch::Launch;
use anyhow::{Context, Result};
use serenity::{
    async_trait,
    builder::CreateEmbed,
    client,
    model::prelude::{Channel, Message},
};

#[async_trait]
pub trait ChannelExt {
    async fn send_embed<F>(&self, ctx: &client::Context, build: F) -> Result<Message>
    where
        F: FnOnce(&mut CreateEmbed) + Send + Sync;
    async fn send_launch(&self, ctx: &client::Context, n: &Launch, r: &String) -> Result<Message>;
    async fn send_apod(&self, ctx: &client::Context, n: &Apod) -> Result<Message>;
}

#[async_trait]
impl ChannelExt for Channel {
    async fn send_embed<F>(&self, ctx: &client::Context, build: F) -> Result<Message>
    where
        F: FnOnce(&mut CreateEmbed) + Send + Sync,
    {
        let mut e = create_basic_embed();
        build(&mut e);

        self.id()
            .send_message(ctx, move |m| {
                m.allowed_mentions(|f| f.replied_user(false));
                m.set_embed(e)
            })
            .await
            .context("Failed to send embed")
    }

    async fn send_launch(&self, ctx: &client::Context, n: &Launch, r: &String) -> Result<Message> {
        let e = create_launch_embed(n, r);

        self.id()
            .send_message(ctx, move |m| m.set_embed(e))
            .await
            .context("Failed to send launch embed")
    }

    async fn send_apod(&self, ctx: &client::Context, n: &Apod) -> Result<Message> {
        let e = create_apod_embed(n);

        self.id()
            .send_message(ctx, move |m| m.set_embed(e))
            .await
            .context("Failed to send apod embed")
    }
}
