use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Mission {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl Default for Mission {
    fn default() -> Self {
        Mission {
            id: 0,
            name: "Unknown".to_string(),
            description: "No description found".to_string(),
        }
    }
}
