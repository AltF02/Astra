use crate::models::traits::ResObject;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use serenity::client;
use serenity::model::channel::Channel;

#[derive(Debug, sqlx::FromRow, sqlx::Type, Clone, Copy, Deserialize, Serialize)]
#[sqlx(transparent)]
pub struct GuildId(pub i64);

#[derive(Debug, sqlx::FromRow, sqlx::Type, Clone, Copy, Deserialize, Serialize)]
#[sqlx(transparent)]
pub struct ChannelId(pub i64);

#[derive(Deserialize, Serialize, Debug)]
pub struct SpaceStationCommon {
    pub id: i32,
    pub url: String,
    pub name: String,
    pub status: Status,
    pub orbit: String,
    pub image_url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ExpeditionCommon {
    pub id: i32,
    pub url: String,
    pub name: String,
    pub start: Option<DateTime<FixedOffset>>,
    pub end: Option<DateTime<FixedOffset>>,
    #[serde(alias = "spacestation")]
    pub space_station: SpaceStationCommon,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ApiResult<T: ResObject> {
    pub count: i32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<T>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Status {
    pub id: i8,
    pub name: String,
    pub abbrev: String,
    pub description: String,
}

impl ChannelId {
    pub async fn fetch(&self, ctx: &client::Context) -> Option<Channel> {
        return match ctx.cache.channel(self.0 as u64).await {
            Some(channel) => Some(channel),
            None => {
                if let Ok(channel) = ctx.http.get_channel(self.0 as u64).await {
                    Some(channel)
                } else {
                    return None;
                }
            }
        };
    }
}
