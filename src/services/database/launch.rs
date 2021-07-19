use super::Interface;
use crate::services::Db;
use chrono::{DateTime, Utc};
use serenity::async_trait;
use sqlx::PgPool;

#[derive(Debug)]
pub struct DBLaunch {
    pub launch_id: String,
    pub name: String,
    pub net: DateTime<Utc>,
    pub vid_url: Option<String>,
    pub image_url: Option<String>,
    pub dispatched: bool,
    pub status: i32,
    pub description: Option<String>,
}

#[async_trait]
impl Interface for DBLaunch {
    async fn get(db: &Db) -> Vec<Self> {
        sqlx::query_as!(
            DBLaunch,
            "SELECT * FROM astra.launches WHERE net > now() ORDER BY net"
        )
        .fetch_all(&db.pool)
        .await
        .unwrap()
    }

    async fn get_limited(db: &Db) -> Vec<Self> {
        sqlx::query_as!(
            DBLaunch,
            "SELECT * FROM astra.launches WHERE net <= (now() + interval '24 hours') AND status = 1;"
        )
        .fetch_all(&db.pool)
        .await
        .unwrap()
    }
}
