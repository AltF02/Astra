use super::Interface;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
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
    async fn get(pool: &PgPool) -> Vec<Self> {
        sqlx::query_as!(
            DBLaunch,
            "SELECT * FROM astra.launches WHERE net > now() ORDER BY net"
        )
        .fetch_all(pool)
        .await
        .unwrap()
    }

    async fn get_limited(pool: &PgPool) -> Vec<Self> {
        sqlx::query_as!(
            DBLaunch,
            "SELECT * FROM astra.launches WHERE net <= (now() + interval '24 hours') AND status = 1;"
        )
        .fetch_all(pool)
        .await
        .unwrap()
    }
}
