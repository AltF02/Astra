use serenity::model::channel::Channel;
use serenity::client;

use crate::services::DB;
use std::fmt::Formatter;

#[derive(Debug, sqlx::FromRow, sqlx::Type)]
pub struct GuildId(pub i64);

#[derive(Debug, sqlx::FromRow)]
pub struct DBGuild {
    pub guild_id: GuildId,
    pub channel_id: i64,
    pub active: bool,
    pub launches: bool,
    pub apod: bool,
    pub events: bool,
}

pub enum Query {
    Launches,
    Events,
    Apod,
}

impl GuildId {
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

impl std::fmt::Display for Query {
    fn fmt<'a>(&self, f: &mut Formatter<'a>) -> std::fmt::Result {
        match self {
            Query::Launches => write!(f, "launches"),
            Query::Events => write!(f, "events"),
            Query::Apod => write!(f, "apod"),
        }
    }
}

impl DB {
    pub async fn get_guilds_queried(&self, active: bool, query: Query) -> Vec<DBGuild> {
        sqlx::query_as(
            format!(
                "SELECT * FROM astra.guilds WHERE active = $1 AND {} = true",
                query
            )
            .as_str(),
        )
        .bind(active)
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }
}
