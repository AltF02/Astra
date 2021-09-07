use chrono::Utc;
use serenity::builder::CreateEmbed;
use serenity::model::channel::EmbedFooter;

use crate::constants::PLACEHOLDER;
use crate::models::launch::Launch;
use crate::models::url::VidURL;

pub async fn create_basic_embed() -> CreateEmbed {
    let mut e = CreateEmbed::default();

    e.timestamp(&Utc::now());
    e
}

pub async fn create_launch_embed(n: &Launch, r: &String) -> CreateEmbed {
    let mut e = create_basic_embed().await;

    e.title(&n.name);
    e.description(format!(
        "> {}",
        &n.mission.clone().unwrap_or_default().description
    ));
    e.field(
        "Rocket",
        format!(
            "➤ Name: **{}**\n➤ Total Launches: **{}**",
            &n.rocket.configuration.name, &n.rocket.configuration.total_launch_count
        ),
        false,
    );
    e.field(
        "Launch",
        format!(
            "➤ Status: **{}**\n➤ Probability: **{}%**",
            &n.status.description,
            &n.probability.unwrap_or(-0)
        ),
        false,
    );
    e.image(
        &n.rocket
            .configuration
            .image_url
            .as_ref()
            .unwrap_or(&PLACEHOLDER.to_string()),
    );
    e.url(&n.vid_urls.get(0).unwrap_or(&VidURL::default()).url);
    e.color(0x00adf8);
    e.footer(|f| f.text(&n.id.to_string()));
    e.author(|a| a.name(format!("Time Remaining: {} hours", r)));
    e
}
