use chrono::Utc;
use serenity::builder::CreateEmbed;

pub async fn create_basic_embed() -> CreateEmbed {
    let mut e = CreateEmbed::default();

    e.timestamp(&Utc::now());
    e
}
