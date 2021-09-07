// Most of this was stolen from https://github.com/unixporn/robbb/blob/master/src/extensions.rs
// The MIT License (MIT)
//
// Copyright (c) 2021 ElKowar
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
// TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
// SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use crate::models::launch::Launch;
use crate::services::{Config, DB};

use crate::bot::embeds::{create_basic_embed, create_launch_embed};
use anyhow::{Context, Result};
use chrono::Duration;
use serenity::builder::CreateEmbed;
use serenity::model::channel::Channel;
use serenity::model::prelude::Message;
use serenity::{async_trait, client};
use std::fmt::Display;
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

#[async_trait]
pub trait MessageExt {
    async fn reply_embed<F>(&self, ctx: &client::Context, build: F) -> Result<Message>
    where
        F: FnOnce(&mut CreateEmbed) + Send + Sync;

    async fn reply_success(
        &self,
        ctx: &client::Context,
        s: impl Display + Send + Sync + 'static,
    ) -> Result<Message>;

    async fn reply_error(
        &self,
        ctx: &client::Context,
        s: impl Display + Send + Sync + 'static,
    ) -> Result<Message>;
}

#[async_trait]
impl MessageExt for Message {
    async fn reply_embed<F>(&self, ctx: &client::Context, build: F) -> Result<Message>
    where
        F: FnOnce(&mut CreateEmbed) + Send + Sync,
    {
        let mut e = create_basic_embed().await;
        build(&mut e);

        self.channel_id
            .send_message(ctx, move |m| {
                m.allowed_mentions(|f| f.replied_user(false));
                m.reference_message(self);
                m.set_embed(e)
            })
            .await
            .context("Failed to send embed")
    }

    async fn reply_success(
        &self,
        ctx: &client::Context,
        s: impl Display + Send + Sync + 'static,
    ) -> Result<Message> {
        self.reply_embed(ctx, |e| {
            e.description(s);
            e.color(0xb8bb26);
        })
        .await
    }

    async fn reply_error(
        &self,
        ctx: &client::Context,
        s: impl Display + Send + Sync + 'static,
    ) -> Result<Message> {
        self.reply_embed(ctx, |e| {
            e.description(s);
            e.color(0xe91714);
        })
        .await
    }
}

#[async_trait]
pub trait ChannelExt {
    async fn send_launch(
        &self,
        ctx: &client::Context,
        n: &Launch,
        r: &String,
    ) -> Result<Message>;
}

#[async_trait]
impl ChannelExt for Channel {
    async fn send_launch(
        &self,
        ctx: &client::Context,
        n: &Launch,
        r: &String,
    ) -> Result<Message> {
        let mut e = create_launch_embed(n, r).await;

        self.id()
            .send_message(ctx, move |m| m.set_embed(e))
            .await
            .context("Failed to send launch embed")
    }
}

pub trait DurationExt {
    fn create_24h(&self) -> String;
}

impl DurationExt for Duration {
    fn create_24h(&self) -> String {
        let mins = (self.num_minutes() - 60 * self.num_hours()).to_string();
        let min = if mins.len() == 1 {
            format!("0{}", mins)
        } else {
            mins
        };
        let hour = if self.num_hours().to_string().len() == 1 {
            format!("0{}", self.num_hours())
        } else {
            self.num_hours().to_string()
        };
        format!("{}:{}", hour, min)
    }
}
