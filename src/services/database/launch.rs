use crate::models::launch::LaunchID;
use crate::services::DB;
use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct DBLaunch {
    pub launch_id: LaunchID,
    pub name: String,
    pub net: DateTime<Utc>,
    pub vid_url: Option<String>,
    pub image_url: Option<String>,
    pub dispatched: bool,
    pub status: i32,
    pub description: Option<String>,
}

impl DB {
    pub async fn get_launch(&self, id: &LaunchID, dispatched: bool) -> Option<DBLaunch> {
        sqlx::query_as(
            "SELECT dispatched, net FROM astra.launches WHERE launch_id = $1 AND dispatched = $2",
        )
        .bind(id)
        .bind(dispatched)
        .fetch_optional(&self.pool)
        .await
        .unwrap()
    }

    pub async fn get_launches(&self) -> Vec<DBLaunch> {
        sqlx::query_as("SELECT * FROM astra.launches WHERE net > now() ORDER BY net")
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }

    pub async fn get_limited_launches(&self) -> Vec<DBLaunch> {
        sqlx::query_as(
            "SELECT * FROM astra.launches WHERE net <= (now() + interval '24 hours') AND status = 1;"
        )
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }
}
