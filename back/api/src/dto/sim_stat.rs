use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    pub value: f32,
    pub decay_rate: Option<f32>,
    #[serde(default)]
    pub order_index: i32,
}
