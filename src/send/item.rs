use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Item {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
}
