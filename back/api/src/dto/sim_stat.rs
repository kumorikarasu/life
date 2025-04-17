use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    pub value: f32,
    pub decay_rate: Option<f32>,
}
