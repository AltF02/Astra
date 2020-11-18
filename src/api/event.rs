use crate::api::common::{ApiResult, SpaceStationCommon};
use crate::api::launch::Launch;
use crate::api::BASE_URL;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Deserialize, Serialize, Debug)]
pub struct Event {
    pub id: i32,
    pub url: String,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub location: String,
    pub news_url: String,
    pub video_url: String,
    pub feature_image: Option<String>,
    pub date: DateTime<FixedOffset>,
    pub launches: Option<Vec<Launch>>,
    // pub expeditions: Option<Vec<Expedition>>,
    pub spacestations: Option<Vec<SpaceStationCommon>>,
}

#[allow(dead_code)]
pub async fn get_next_event() -> Result<ApiResult<Event>, Box<dyn Error>> {
    let res = reqwest::get(&format!("{}/event/upcoming/?format=json", BASE_URL))
        .await?
        .json::<ApiResult<Event>>()
        .await?;

    Ok(res)
}
