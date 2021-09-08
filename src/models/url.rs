use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct VidURL {
    pub priority: i8,
    pub title: String,
    pub description: String,
    pub feature_image: Option<String>,
    pub url: String,
}
