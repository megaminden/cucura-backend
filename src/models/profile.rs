use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub profile_type: Option<String>,
    pub business: Option<String>,
    pub interests: Option<String>,
}

impl Profile {
    pub fn new(username: String, email: String) -> Profile {
        Profile {
            id: None,
            username,
            email,
            bio: None,
            profile_type: None,
            business: None,
            interests: None,
        }
    }

    //     create new with complete constructor
    #[allow(dead_code)]
    pub fn default(
        id: Option<ObjectId>,
        username: String,
        email: String,
        bio: Option<String>,
        profile_type: Option<String>,
        business: Option<String>,
        interests: Option<String>,
    ) -> Profile {
        Profile {
            id,
            username,
            email,
            bio,
            profile_type,
            business,
            interests,
        }
    }
}
