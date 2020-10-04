use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::api::crew::Crew;
use crate::api::spacecraft::SpaceCraft;

#[derive(Deserialize, Serialize, Debug)]
pub struct Rocket {
    pub id: i32,
    pub configuration: RocketConfiguration
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RocketConfiguration {
    pub id: i32,
    pub launch_library_id: i32,
    pub url: String,
    pub name: String,
    pub description: String,
    pub family: String,
    pub full_name: String,
    pub program: String,
    pub variant: String,
    pub alias: String,
    pub min_stage: i8,
    pub max_stage: i8,
    pub length: f32,
    pub diameter: f32,
    pub launch_mass: i16,
    pub leo_capacity: i32,
    pub gto_capacity: i32,
    pub to_thrust: i32,
    pub image_url: String,
    pub wiki_url: String,
    pub total_launch_count: i32,
    pub consecutive_successful_launches: i32,
    pub successful_launches: i32,
    pub failed_launches: i32,
    pub pending_launches: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RocketSpaceCraftStage {
    pub id: i32,
    pub url: String,
    pub mission_end: Option<DateTime<Utc>>,
    pub destination: String,
    pub launch_crew: Vec<Crew>,
    pub landing_crew: Vec<Crew>,
    pub spacecraft: SpaceCraft,
}
