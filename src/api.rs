pub mod common;
pub mod launch;
pub mod traits;
pub mod event;
pub mod launcher;
pub mod manufacturer;
pub mod rocket;
pub mod crew;
pub mod spacecraft;

#[allow(unused_must_use)]
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Debug;
use chrono::{DateTime, FixedOffset};
use crate::api::common::{SpaceStationCommon, Status, ApiResult};

#[cfg(debug_assertions)]
pub const BASE_URL: &str = "https://lldev.thespacedevs.com/2.0.0";

#[cfg(not(debug_assertions))]
pub const BASE_URL: &str = "https://ll.thespacedevs.com/2.0.0";

