use crate::models::common::{ChannelId, GuildId};
use crate::services::DB;
use anyhow::Result;
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

impl Query {
    pub fn from_str(i: &str) -> Option<Self> {
        match i {
            "apod" => Some(Query::Apod),
            "launches" => Some(Query::Launches),
            "events" => Some(Query::Events),
            &_ => None,
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

    pub async fn set_guild_channel(&self, channel_id: i64, guild_id: i64) -> Result<()> {
        sqlx::query(
            "INSERT INTO astra.guilds (guild_id, channel_id) VALUES ($1, $2) ON CONFLICT (guild_id) DO UPDATE SET channel_id = $2, active = true"
        )
        .bind(guild_id)
        .bind(channel_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn toggle_guild_setting(&self, guild_id: i64, query: Query) -> Result<()> {
        sqlx::query(
            format!(
                "UPDATE astra.guilds SET {q} = NOT {q} WHERE guild_id = $1",
                q = query
            )
            .as_str(),
        )
        .bind(guild_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
