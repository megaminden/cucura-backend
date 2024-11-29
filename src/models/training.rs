// models/training.rs
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize)]
pub struct Training {
    pub training_id: Uuid,
    pub trainer_id: Uuid,
    pub title: String,
    pub description: String,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub duration: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Training {
    pub fn new(
        trainer_id: Uuid,
        title: String,
        description: String,
        start_date: Option<NaiveDateTime>,
        end_date: Option<NaiveDateTime>,
        duration: String,
    ) -> Training {
        Training {
            training_id: Uuid::new_v4(),
            trainer_id,
            title,
            description,
            start_date,
            end_date,
            duration,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}
