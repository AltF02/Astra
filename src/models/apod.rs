use crate::constants::APOD_URL;
use crate::services::database::apod::DBApod;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Apod {
    pub copyright: Option<String>,
    pub date: String,
    pub explanation: String,
    pub hdurl: String,
    pub title: String,
}

impl Apod {
    pub async fn fetch(key: &String) -> Result<Self, ApodError> {
        let res = match reqwest::get(format!("{}{}", APOD_URL, key)).await {
            Ok(res) => res,
            Err(e) => return Err(ApodError::new(format!("Reqwest error {}", e).as_str())),
        };

        if !res.status().is_success() {
            return Err(ApodError::new(
                format!("Status code incorrect: {}", res.status().as_u16()).as_str(),
            ));
        }

        return match res.json::<Apod>().await {
            Ok(apod) => Ok(apod),
            Err(e) => Err(ApodError::new(
                format!("Reqwest json error: {}", e).as_str(),
            )),
        };
    }
}

impl From<DBApod> for Apod {
    fn from(a: DBApod) -> Self {
        Apod {
            copyright: a.copyright,
            date: a.publish_date.to_string(),
            explanation: a.explanation,
            hdurl: a.hdurl,
            title: a.title,
        }
    }
}

#[derive(Debug)]
pub struct ApodError {
    details: String,
}

impl ApodError {
    fn new(msg: &str) -> ApodError {
        ApodError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ApodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ApodError {
    fn description(&self) -> &str {
        &self.details
    }
}
