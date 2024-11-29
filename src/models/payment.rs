use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::payment_type::PaymentType;
#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    pub payment_id: Uuid,
    pub purchaser_id: Uuid,
    pub seller_id: Uuid,
    pub payment_type: PaymentType, // card, cash, bank transfer
    pub description: Option<String>,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Payment {
    pub fn new(
        purchaser_id: Uuid,
        seller_id: Uuid,
        payment_type: PaymentType,
        description: Option<String>,
        amount: f64,
        currency: String,
        status: String,
    ) -> Payment {
        Payment {
            payment_id: Uuid::new_v4(),
            purchaser_id,
            seller_id,
            payment_type,
            description,
            amount,
            currency,
            status,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}
