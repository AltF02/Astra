use crate::api::launch::Launch;
use crate::api::traits::ResObject;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SpaceStationCommon {
    pub id: i32,
    pub url: String,
    pub name: String,
    pub status: Status,
    pub orbit: String,
    pub image_url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ExpeditionCommon {
    pub id: i32,
    pub url: String,
    pub name: String,
    pub start: Option<DateTime<FixedOffset>>,
    pub end: Option<DateTime<FixedOffset>>,
    pub spacestation: SpaceStationCommon
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ApiResult<T: ResObject> {
    pub count: i32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<T>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Status {
    pub id: i8,
    pub name: String
}