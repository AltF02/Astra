use crate::services::DB;
use std::fmt::Formatter;

#[derive(Debug, sqlx::FromRow)]
pub struct DBGuild {
    pub guild_id: i64,
    pub channel_id: i64,
    pub active: bool,
    pub launches: bool,
    pub apod: bool,
    pub events: bool,
}

pub enum Query {
    LAUNCHES,
    EVENTS,
    APOD,
}

impl std::fmt::Display for Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Query::LAUNCHES => write!(f, "launches"),
            Query::EVENTS => write!(f, "events"),
            Query::APOD => write!(f, "apod"),
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
