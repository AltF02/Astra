use serde::{Deserialize, Serialize};
use chrono::{Date, Utc, DateTime};

#[derive(Deserialize, Serialize, Debug)]
pub struct Crew {
    pub id: i32,
    pub role: String,
    pub astronaut: Astronaut
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Astronaut {
    pub id: i32,
    pub url: String,
    pub name: String,
    pub status: String,
    // pub date_of_birth: Date<Utc>,
    // pub date_of_death: Option<Date<Utc>>,
    pub nationality: String,
    pub twitter: Option<String>,
    pub instagram: Option<String>,
    pub bio: String,
    pub profile_image: String,
    pub wiki: String,
    pub last_flight: Option<DateTime<Utc>>,
    pub first_flight: Option<DateTime<Utc>>,
}