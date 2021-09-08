use crate::models::common::{ChannelId, GuildId};
use crate::services::DB;
use std::fmt::Formatter;

#[derive(Debug, sqlx::FromRow)]
pub struct DBGuild {
    pub guild_id: GuildId,
    pub channel_id: ChannelId,
    pub active: bool,
    pub launches: bool,
    pub apod: bool,
    pub events: bool,
}

#[allow(dead_code)]
pub enum Query {
    Launches,
    Events,
    Apod,
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
