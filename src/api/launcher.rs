use crate::api::manufacturer::Manufacturer;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct LauncherCommon {
    pub id: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LauncherCommonConfiguration {
    pub id: i32,
    pub launch_library_id: i32,
    pub url: String,
    pub name: String,
    pub family: String,
    pub full_name: String,
    pub variant: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Launcher {
    pub id: i32,
    pub launch_library_id: i32,
    pub url: String,
    pub name: String,
    pub description: String,
    pub family: String,
    pub full_name: String,
    pub manufacturer: Manufacturer,
    pub variant: String,
    pub alias: String,
    pub min_stage: String,
    pub max_stage: String,
    pub length: f32,
    pub diameter: f32,
    pub maiden_flight: String,
    pub launch_mass: i32,
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

