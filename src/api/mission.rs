use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Mission {
    pub id: i32,
    pub name: String,
    pub description: String,
}
