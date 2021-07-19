pub mod guild;
pub mod launch;

use anyhow::*;
use serenity::async_trait;
use serenity::prelude::TypeMapKey;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool};
use std::sync::Arc;

pub struct Db {
    pub pool: PgPool,
}

impl TypeMapKey for Db {
    type Value = Arc<Db>;
}

impl Db {
    pub async fn new(uri: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .connect(uri)
            .await?;
        Ok(Self { pool })
    }
}

#[async_trait]
pub trait Interface {
    async fn get(_pool: &Db) -> Vec<Self>
    where
        Self: Sized,
    {
        unimplemented!()
    }
    async fn get_limited(_pool: &Db) -> Vec<Self>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}
