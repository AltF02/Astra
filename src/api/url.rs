use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct VidURL {
    pub priority: i8,
    pub title: String,
    pub description: String,
    pub feature_image: String,
    pub url: String,
}
