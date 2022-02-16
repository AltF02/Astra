use serenity::model::prelude::{Activity, OnlineStatus, Ready};
use serenity::model::Permissions;
use serenity::prelude::Context;

pub struct ReadyEvent;

impl ReadyEvent {
    pub async fn run(ctx: &Context, ready: &Ready) {
        let perms = Permissions::from_bits(0).unwrap();
        let user = &ready.user;
        ctx.set_presence(
            Some(Activity::listening("new launch announcements")),
            OnlineStatus::Online,
        )
        .await;

        println!(
            "
            Ready as {}
             * Serving {} guilds
             * Invite URL: {}",
            user.tag(),
            ready.guilds.len(),
            user.invite_url(ctx.clone(), perms).await.unwrap(),
        );
    }
}
