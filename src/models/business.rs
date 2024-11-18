use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Business {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: String, // This is the user_id of the user who created the business
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
