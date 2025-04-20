use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Model {
    id: i32,
    name: String,
    user_id: i32,
    stats: Vec<crate::dto::sim_stat::Model>,
}

impl Model {
    pub fn new(id: i32, name: String, user_id: i32, stats: Vec<crate::dto::sim_stat::Model>) -> Model {
        Model {
            id,
            name,
            user_id,
            stats,
        }
    }
}
