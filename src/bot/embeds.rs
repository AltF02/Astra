use chrono::Utc;
use serenity::builder::CreateEmbed;

use crate::constants::PLACEHOLDER;
use crate::models::apod::Apod;
use crate::models::launch::Launch;
use crate::models::url::VidURL;

pub fn create_basic_embed() -> CreateEmbed {
    let mut e = CreateEmbed::default();

    e.timestamp(&Utc::now());
    e
}

pub fn create_launch_embed(n: &Launch, r: &String) -> CreateEmbed {
    let mut e = create_basic_embed();

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

pub fn create_apod_embed(a: &Apod) -> CreateEmbed {
    let mut e = create_basic_embed();

    e.title(&a.title);
    e.image(&a.hdurl);
    e.description(format!("> {}", &a.explanation));
    e.footer(|f| {
        f.text(format!(
            "Copyright © {}. All Rights Reserved.",
            &a.copyright.as_ref().unwrap_or(&"NASA".to_string())
        ))
    });
    e.color(0x5694c7);
    e
}
