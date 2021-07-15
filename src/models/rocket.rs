use crate::models::crew::Crew;
use crate::models::spacecraft::SpaceCraft;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Rocket {
    pub id: i32,
    pub configuration: RocketConfiguration,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RocketConfiguration {
    pub id: i32,
    pub url: String,
    pub name: String,
    pub description: String,
    pub family: String,
    pub full_name: String,
    // pub program: Vec<String>,
    pub variant: String,
    pub alias: String,
    pub min_stage: Option<i8>,
    pub max_stage: Option<i8>,
    pub length: Option<f32>,
    pub diameter: Option<f32>,
    pub launch_mass: Option<i32>,
    pub leo_capacity: Option<i32>,
    pub gto_capacity: Option<i32>,
    pub to_thrust: Option<i32>,
    pub image_url: Option<String>,
    pub wiki_url: Option<String>,
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
