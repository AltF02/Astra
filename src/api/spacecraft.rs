use crate::api::common::Status;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SpaceCraft {
    pub id: i32,
    pub url: String,
    pub name: String,
    pub serial_number: String,
    pub status: Status,
    pub description: String,
    pub spacecraft_config: SpaceCraftConfig,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SpaceCraftConfig {
    pub id: i32,
    pub url: String,
    pub name: String,
    // agency: Agency TODO Make this
    pub in_use: bool,
    pub capability: String,
    pub history: String,
    pub details: String,
    // maiden_flight: Option<Date<Utc>>,
    pub height: Option<f32>,
    pub diameter: Option<f32>,
    pub human_rated: bool,
    pub crew_capacity: Option<i8>,
    pub payload_capacity: Option<i32>,
    pub flight_life: String,
    pub image_url: Option<String>,
    pub nation_url: Option<String>,
    pub wiki_link: Option<String>,
    pub info_link: String,
}
