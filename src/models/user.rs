use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub user_type: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn new(username: String, email: String, password: String, user_type: String) -> User {
        User {
            user_id: Uuid::new_v4(),
            username,
            email,
            password,
            user_type,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub user_type: String,
}
impl NewUser {
    pub fn new(username: String, email: String, password: String, user_type: String) -> NewUser {
        NewUser {
            username,
            email,
            password,
            user_type,
        }
    }

    pub fn to_user(&self) -> User {
        User::new(
            self.username.clone(),
            self.email.clone(),
            self.password.clone(),
            self.user_type.clone(),
        )
    }
}
