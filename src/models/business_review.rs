use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::rating::Rating;

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessReview {
    pub business_review_id: Uuid,
    pub reviewer_id: Uuid,
    pub business_id: Uuid,
    pub rating: Rating,
    pub comment: Option<String>,
    pub review_link: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
impl BusinessReview {
    pub fn new(
        business_review_id: Uuid,
        reviewer_id: Uuid,
        business_id: Uuid,
        rating: Rating,
        comment: Option<String>,
        review_link: Option<String>,
    ) -> BusinessReview {
        BusinessReview {
            business_review_id,
            reviewer_id,
            business_id,
            rating,
            comment,
            review_link,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
