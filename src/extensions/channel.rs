use crate::bot::components::create_launch_components;
use crate::bot::embeds::{create_apod_embed, create_basic_embed, create_launch_embed};
use crate::models::apod::Apod;
use crate::models::launch::Launch;
use serenity::Result;
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
    }

    async fn send_launch(&self, ctx: &client::Context, n: &Launch, r: &String) -> Result<Message> {
        let e = create_launch_embed(n, r);
        let c = create_launch_components(&n.id);

        self.id()
            .send_message(ctx, move |m| {
                m.set_embed(e);
                m.set_components(c);
                m
            })
            .await
    }

    async fn send_apod(&self, ctx: &client::Context, n: &Apod) -> Result<Message> {
        let e = create_apod_embed(n);

        self.id().send_message(ctx, move |m| m.set_embed(e)).await
    }
}
