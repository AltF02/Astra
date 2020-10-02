#[allow(unused_must_use)]
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Debug;

pub const BASE_URL: &str = "https://launchlibrary.net/1.4";

#[derive(Deserialize, Serialize, Debug)]
pub struct Rockets {
    pub rockets: Vec<Rocket>,
    pub total: i32,
    pub count: i32,
    pub offset: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rocket {
    pub id: i32,
    pub name: String,
    pub configuration: String,
    pub family: Option<Family>,
    #[serde(alias = "familyname")]
    pub family_name: Option<String>,
    #[serde(alias = "wikiURL")]
    pub wiki_url: String,
    #[serde(alias = "imageURL")]
    pub image_url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Family {
    pub id: i32,
    pub name: String,
    pub agencies: String,
}

// ===========================================

#[derive(Deserialize, Serialize, Debug)]
pub struct Agencies {
    pub agencies: Vec<Agency>,
    pub total: i32,
    pub count: i32,
    pub offset: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Agency {
    pub id: i32,
    pub name: String,
    #[serde(alias = "countryCode")]
    pub country_code: String,
    pub abbrev: String,
    #[serde(alias = "type")]
    pub agency_type: i32,
    // #[serde(alias = "infoURL")]
    // pub info_url: String,
    #[serde(alias = "wikiURL")]
    pub wiki_url: String,
    #[serde(alias = "infoURLs")]
    pub info_urls: Vec<String>,
}

// ===========================================

#[derive(Deserialize, Serialize, Debug)]
pub struct Launches {
    pub launches: Vec<Launch>,
    pub total: i32,
    pub offset: i32,
    pub count: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Launch {
    pub id: i32,
    pub name: String,
    #[serde(alias = "windowstart")]
    pub window_start: String,
    #[serde(alias = "windowend")]
    pub window_end: String,
    pub wsstamp: i64,
    pub westamp: i64,
    pub netstamp: i64,
    pub tbdtime: i8,
    pub status: i32,
    #[serde(alias = "vidURLs")]
    pub vid_urls: Vec<String>,
    pub probability: i8,
    pub rocket: Rocket,
    pub missions: Vec<Mission>,
    pub lsp: Agency,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Mission {
    pub id: i32,
    pub name: String,
    pub description: String,
    #[serde(alias = "wikiURL")]
    pub wiki_url: String,
}

pub trait Ctx: std::fmt::Display {}

impl Ctx for str {}
impl Ctx for String {}
impl Ctx for i32 {}

pub async fn get_rocket<'a, T: Ctx + ?Sized>(ctx: &'a T) -> Result<Rockets, Box<dyn Error>> {
    let res = reqwest::get(&format!("{}/rocket/{}", BASE_URL, ctx))
        .await?
        .json::<Rockets>()
        .await?;

    Ok(res)
}

pub async fn get_agency<'a, T: Ctx + ?Sized>(ctx: &'a T) -> Result<Agencies, Box<dyn Error>> {
    let res = reqwest::get(&format!("{}/agency/{}", BASE_URL, ctx))
        .await?
        .json::<Agencies>()
        .await?;

    Ok(res)
}

pub async fn get_next_launch<'a>() -> Result<Launches, Box<dyn Error>> {
    let res = reqwest::get(&format!("{}/launch/next/5", BASE_URL))
        .await?
        .json::<Launches>()
        .await?;

    Ok(res)
}
