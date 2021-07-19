use crate::services::{Config, Db};

use serenity::{async_trait, client};
use std::sync::Arc;

#[async_trait]
pub trait ClientContextExt {
    async fn get_config(&self) -> Arc<Config>;
    async fn get_db(&self) -> Arc<Db>;
}

#[async_trait]
impl ClientContextExt for client::Context {
    async fn get_config(&self) -> Arc<Config> {
        self.data.read().await.get::<Config>().unwrap().clone()
    }

    async fn get_db(&self) -> Arc<Db> {
        self.data.read().await.get::<Db>().unwrap().clone()
    }
}
