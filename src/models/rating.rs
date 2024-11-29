// models/rating.rs

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Rating {
    pub rating_id: Uuid,
    pub score: i32,
    pub description: Option<String>,
}

impl Rating {
    pub fn new(score: i32) -> Rating {
        Rating {
            rating_id: Uuid::new_v4(),
            score,
            description: None,
        }
    }
}
