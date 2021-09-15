use crate::services::{Config, DB};
use serenity::{async_trait, client};
use std::sync::Arc;

#[async_trait]
pub trait ClientContextExt {
    async fn get_config(&self) -> Arc<Config>;
    async fn get_db(&self) -> Arc<DB>;
    async fn get_db_and_config(&self) -> (Arc<DB>, Arc<Config>);
}

#[async_trait]
impl ClientContextExt for client::Context {
    async fn get_config(&self) -> Arc<Config> {
        self.data.read().await.get::<Config>().unwrap().clone()
    }

    async fn get_db(&self) -> Arc<DB> {
        self.data.read().await.get::<DB>().unwrap().clone()
    }

    async fn get_db_and_config(&self) -> (Arc<DB>, Arc<Config>) {
        let db = self.get_db().await;
        let config = self.get_config().await;
        (db, config)
    }
}
