use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentType {
    pub payment_type_id: Uuid,
    pub payment_type: String,
}
impl PaymentType {
    pub fn new(payment_type: String) -> PaymentType {
        PaymentType {
            payment_type_id: Uuid::new_v4(),
            payment_type,
        }
    }
}
