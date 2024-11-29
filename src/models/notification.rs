use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NotificationType {
    #[default]
    SomeoneSentMessage,
    SomeoneLikedPost,
    SomeoneViewedProfile,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Notification {
    pub notification_type: NotificationType,
    pub user_id: Uuid,
    pub message: String,
    pub confirmed: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Notification {
    pub fn new(
        notification_type: NotificationType,
        user_id: Uuid,
        message: String,
    ) -> Notification {
        Notification {
            notification_type,
            user_id,
            message,
            confirmed: false,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}
