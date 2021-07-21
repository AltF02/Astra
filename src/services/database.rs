pub mod guild;
pub mod launch;

use anyhow::*;
use serenity::prelude::TypeMapKey;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::Arc;

pub struct DB {
    pub pool: PgPool,
}

impl TypeMapKey for DB {
    type Value = Arc<DB>;
}

impl DB {
    pub async fn new(uri: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .connect(uri)
            .await?;
        Ok(Self { pool })
    }

    pub async fn run_migrations(&self) -> Result<()> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .context("Failed to run database migrations")?;

        Ok(())
    }
}
