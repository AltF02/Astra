pub mod guild;
pub mod launch;

use async_trait::async_trait;
use serenity::prelude::TypeMapKey;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Pool, Postgres};

pub struct ConnectionPool;

impl TypeMapKey for ConnectionPool {
    type Value = PgPool;
}

#[async_trait]
pub trait Interface {
    async fn get(pool: &PgPool) -> Vec<Self>
    where
        Self: Sized;
    async fn get_limited(pool: &PgPool) -> Vec<Self>
    where
        Self: Sized;
}

pub async fn connect(
    uri: &str,
) -> Result<Pool<Postgres>, Box<dyn std::error::Error + Send + Sync>> {
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(uri)
        .await?;

    Ok(pool)
}
