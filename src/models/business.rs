use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Business {
    pub business_id: Uuid,
    pub user_ids: Option<Vec<Uuid>>, // This is the user_id of the user who created the business
    pub name: String,
    pub description: String,
    pub logo: Option<String>,
    pub pictures: Option<Vec<String>>,
    pub founder: String,
    pub industry: String,
    pub phone: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub country: String,
    pub website: Option<String>,
    pub contact_email: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Business {
    pub fn new(
        user_ids: Option<Vec<Uuid>>,
        name: String,
        description: String,
        founder: String,
        industry: String,
        phone: String,
        country: String,
    ) -> Business {
        Business {
            business_id: Uuid::new_v4(),
            user_ids,
            name,
            description,
            logo: None,
            pictures: None,
            founder,
            industry,
            phone,
            address: None,
            city: None,
            region: None,
            country,
            website: None,
            contact_email: None,
            created_at: Utc::now().to_string(),
            updated_at: Utc::now().to_string(),
        }
    }
}
