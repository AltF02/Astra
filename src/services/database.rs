use chrono::{DateTime, Utc};
use serenity::prelude::TypeMapKey;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Pool, Postgres};

pub struct ConnectionPool;

impl TypeMapKey for ConnectionPool {
    type Value = PgPool;
}

pub struct DBLaunch {
    pub launch_id: String,
    pub name: String,
    pub net: DateTime<Utc>,
    pub tbd: bool,
    pub vid_url: Option<String>,
    pub image_url: Option<String>,
    pub dispatched: bool,
    pub status: i32,
}

/*
Status diagram:
    1: GO,
    2: TBD,
    3: Success,
    4: Failure
*/

pub(crate) async fn connect(
    uri: &String,
) -> Result<Pool<Postgres>, Box<dyn std::error::Error + Send + Sync>> {
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(uri)
        .await?;

    Ok(pool)
}

pub(crate) async fn get_launch_database(pool: &PgPool) -> Vec<DBLaunch> {
    sqlx::query_as!(DBLaunch, "SELECT * FROM apollo.launches WHERE net < 'tomorrow'::timestamp with time zone AND status = 1;").fetch_all(pool).await.unwrap()
}
