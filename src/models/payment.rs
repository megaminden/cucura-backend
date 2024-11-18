// models/payment.rs
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub purchaser_id: String,
    pub seller_id: String,
    pub payment_type: String, // card, cash, bank transfer
    pub description: Option<String>,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}
