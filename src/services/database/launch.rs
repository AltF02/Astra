use crate::models::launch::{Launch, LaunchID};
use crate::services::DB;
use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::postgres::PgQueryResult;

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

impl From<Launch> for DBLaunch {
    fn from(l: Launch) -> Self {
        DBLaunch {
            launch_id: l.id,
            name: l.name,
            net: l.net,
            vid_url: l.vid_urls.get(0).map(|v| v.url.to_owned()),
            image_url: l.rocket.configuration.image_url,
            dispatched: false,
            status: l.status.id as i32,
            description: l.mission.as_ref().map(|m| m.description.to_owned()),
        }
    }
}

impl DB {
    pub async fn get_launch(&self, id: &LaunchID, dispatched: bool) -> Option<DBLaunch> {
        sqlx::query_as("SELECT * FROM astra.launches WHERE launch_id = $1 AND dispatched = $2")
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

    pub async fn set_net(&self, launch: &Launch) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("UPDATE astra.launches SET net = $1 WHERE launch_id = $2")
            .bind(&launch.net)
            .bind(&launch.id)
            .execute(&self.pool)
            .await
    }

    pub async fn set_launch(&self, launch: DBLaunch) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query(
            "INSERT INTO astra.launches (launch_id, name, net, vid_url, \
                image_url, dispatched, status, description) \
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8) \
                    ON CONFLICT (launch_id) DO \
                        UPDATE SET net = $3, vid_url = $4, dispatched = $6, \
                        status = $7, description = $8;",
        )
        .bind(launch.launch_id)
        .bind(launch.name)
        .bind(launch.net)
        .bind(launch.vid_url)
        .bind(launch.image_url)
        .bind(launch.dispatched)
        .bind(launch.status)
        .bind(launch.description)
        .execute(&self.pool)
        .await
    }
}
