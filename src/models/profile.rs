use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub profile_id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub profile_type: Option<String>,
    pub business: Option<String>,
    pub interests: Option<Vec<String>>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Profile {
    pub fn new(user_id: Uuid, email: String, username: String) -> Profile {
        Profile {
            profile_id: Uuid::new_v4(),
            user_id,
            username,
            email,
            bio: None,
            profile_type: None,
            business: None,
            interests: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }

    pub fn new_with_details(
        user_id: Uuid,
        email: String,
        username: String,
        bio: String,
        profile_type: String,
        business: String,
        interests: Vec<String>,
    ) -> Profile {
        Profile {
            profile_id: Uuid::new_v4(),
            user_id,
            username,
            email,
            bio: Some(bio),
            profile_type: Some(profile_type),
            business: Some(business),
            interests: Some(interests),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}
