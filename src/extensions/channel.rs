use crate::bot::embeds::create_launch_embed;
use crate::models::launch::Launch;
use anyhow::{Context, Result};
use serenity::{
    async_trait, client,
    model::prelude::{Channel, Message},
};

#[async_trait]
pub trait ChannelExt {
    async fn send_launch(&self, ctx: &client::Context, n: &Launch, r: &String) -> Result<Message>;
}

#[async_trait]
impl ChannelExt for Channel {
    async fn send_launch(&self, ctx: &client::Context, n: &Launch, r: &String) -> Result<Message> {
        let e = create_launch_embed(n, r).await;

        self.id()
            .send_message(ctx, move |m| m.set_embed(e))
            .await
            .context("Failed to send launch embed")
    }
}
