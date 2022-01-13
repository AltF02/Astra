use crate::bot::embeds::create_reminder_embed;
use crate::services::database::launch::DBLaunch;
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::prelude::User;
use serenity::Result;

#[async_trait]
pub trait UserExt {
    async fn send_reminder(
        &self,
        ctx: &Context,
        user: &User,
        msg: &str,
        launch: &DBLaunch,
    ) -> Result<Message>;
}

#[async_trait]
impl UserExt for User {
    async fn send_reminder(
        &self,
        ctx: &Context,
        user: &User,
        msg: &str,
        launch: &DBLaunch,
    ) -> Result<Message> {
        let e = create_reminder_embed(user, msg, launch);
        self.direct_message(ctx, move |m| m.set_embed(e)).await
    }
}
