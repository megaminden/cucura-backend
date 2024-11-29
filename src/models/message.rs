use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub message_id: Uuid,
    pub sender: Uuid,
    pub receiver: Uuid,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Message {
    pub fn new(sender: Uuid, receiver: Uuid, content: String) -> Message {
        Message {
            message_id: Uuid::new_v4(),
            sender,
            receiver,
            content,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
