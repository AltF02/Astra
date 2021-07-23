use crate::constants::BASE_URL;
use crate::models::common::{ApiResult, Status};
use crate::models::mission::Mission;
use crate::models::rocket::Rocket;
use crate::models::url::VidURL;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;

pub type LaunchID = String;

#[derive(Deserialize, Serialize, Debug)]
pub struct Launch {
    pub id: LaunchID,
    pub url: String,
    pub slug: String,
    pub name: String,
    pub status: Status,
    pub net: DateTime<Utc>,
    pub window_end: DateTime<Utc>,
    pub window_start: DateTime<Utc>,
    pub probability: Option<i8>,
    pub holdreason: Option<String>,
    pub failreason: Option<String>,
    pub rocket: Rocket,
    pub mission: Option<Mission>,
    // #[serde(alias = "infoURLs")]
    // pub info_urls: Vec<String>,
    #[serde(alias = "vidURLs")]
    pub vid_urls: Vec<VidURL>,
}
impl Launch {
    pub async fn get_next_launch<'a>() -> Result<ApiResult<Launch>, Box<dyn Error>> {
        let res = reqwest::get(&format!(
            "{}/launch/upcoming/?format=json&mode=detailed",
            BASE_URL
        ))
        .await?
        .json::<ApiResult<Launch>>()
        .await?;

        Ok(res)
    }
}
