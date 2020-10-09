pub mod common;
pub mod crew;
pub mod event;
pub mod launch;
pub mod launcher;
pub mod manufacturer;
pub mod mission;
pub mod rocket;
pub mod spacecraft;
pub mod traits;
pub mod url;

use crate::api::common::{ApiResult, SpaceStationCommon, Status};
use chrono::{DateTime, FixedOffset};
#[allow(unused_must_use)]
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Debug;

#[cfg(debug_assertions)]
pub const BASE_URL: &str = "https://lldev.thespacedevs.com/2.0.0";

#[cfg(not(debug_assertions))]
pub const BASE_URL: &str = "https://ll.thespacedevs.com/2.0.0";
