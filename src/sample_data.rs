pub mod models;
pub use crate::models::{
    business::Business, message::Message, notification::Notification, payment::Payment,
    payment_type::PaymentType, profile::Profile, training::Training, user::User,
};

use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use dotenv::dotenv;
use fake::{
    faker::address::en::CountryName, faker::company::en::CompanyName,
    faker::internet::en::SafeEmail, faker::internet::en::Username, faker::lorem::en::Sentence,
    faker::name::en::Name, Fake,
};
use models::notification::NotificationType;
use mongodb::Client;
use std::env;

use rand::seq::SliceRandom;
use uuid::Uuid;

use actix_web::{web, App, HttpServer};
use mongodb::bson::doc;
use std::ops::Range;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv().ok();

    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    let client = Client::with_uri_str(uri).await.unwrap();
    let db = client.database("cucura-ccdb");

    // Load sample users
    let user_collection = db.collection::<User>("users");
    let hashed_password = hash("password".to_string(), DEFAULT_COST).unwrap();
    let sample_users = (1..=12)
        .map(|_| {
            User::new(
                Username().fake(),
                SafeEmail().fake(),
                hashed_password.clone(),
                if bool::default().fake() {
                    "Founder".to_string()
                } else {
                    "Partner".to_string()
                },
            )
        })
        .collect::<Vec<_>>();
    for user in &sample_users {
        user_collection.insert_one(user).await.unwrap();
    }

    // Collect user IDs
    let user_ids: Vec<Uuid> = sample_users.iter().map(|user| user.user_id).collect();

    // Load sample profiles
    let profile_collection = db.collection::<Profile>("profiles");
    let sample_profiles = user_ids
        .iter()
        .map(|&user_id| Profile::new(user_id.clone(), Username().fake(), SafeEmail().fake()))
        .collect::<Vec<_>>();
    for profile in sample_profiles {
        profile_collection.insert_one(profile).await.unwrap();
    }

    // Load sample payments
    let payment_collection = db.collection::<Payment>("payments");
    let sample_payments = (1..=12)
        .map(|_| {
            Payment::new(
                *user_ids.choose(&mut rand::thread_rng()).unwrap(),
                *user_ids.choose(&mut rand::thread_rng()).unwrap(),
                if bool::default().fake() {
                    PaymentType::new("CREDIT_CARD".to_string())
                } else {
                    PaymentType::new("CASH".to_string())
                },
                Some(Sentence(1..3).fake()),
                (10.0..100.0).fake::<f64>(),
                "USD".to_string(),
                if bool::default().fake() {
                    "COMPLETED".to_string()
                } else {
                    "PENDING".to_string()
                },
            )
        })
        .collect::<Vec<_>>();
    for payment in sample_payments {
        payment_collection.insert_one(payment).await.unwrap();
    }

    // Load sample trainings
    let training_collection = db.collection::<Training>("trainings");
    let sample_trainings = (1..=12)
        .map(|_| {
            Training::new(
                *user_ids.choose(&mut rand::thread_rng()).unwrap(),
                Sentence(1..3).fake(),
                Sentence(3..5).fake(),
                Some((Utc::now() - chrono::Duration::days(30)).naive_utc()),
                Some((Utc::now() + chrono::Duration::days(30)).naive_utc()),
                format!("{} hours", (1..5).fake::<u8>()),
            )
        })
        .collect::<Vec<_>>();
    for training in sample_trainings {
        training_collection.insert_one(training).await.unwrap();
    }

    // Load sample businesses
    let business_collection = db.collection::<Business>("businesses");
    let sample_businesses = (1..=12)
        .map(|_| {
            Business::new(
                Some(vec![
                    *user_ids.choose(&mut rand::thread_rng()).unwrap(),
                    *user_ids.choose(&mut rand::thread_rng()).unwrap(),
                ]),
                CompanyName().fake(),
                Sentence(3..5).fake(),
                Name().fake(),
                Sentence(1..2).fake(),
                format!("772-456-78{:02}", (1..=12).fake::<u8>()),
                CountryName().fake(),
            )
        })
        .collect::<Vec<_>>();
    for business in sample_businesses {
        business_collection.insert_one(business).await.unwrap();
    }

    // Load sample messages
    let message_collection = db.collection::<Message>("messages");
    let sample_messages = (1..=12)
        .map(|_| {
            Message::new(
                *user_ids.choose(&mut rand::thread_rng()).unwrap(),
                *user_ids.choose(&mut rand::thread_rng()).unwrap(),
                Sentence(3..15).fake(),
            )
        })
        .collect::<Vec<_>>();
    for message in sample_messages {
        message_collection.insert_one(message).await.unwrap();
    }

    // Load sample notifications
    let notification_collection = db.collection::<Notification>("notifications");
    let sample_notifications = (1..=12)
        .map(|_| {
            Notification::new(
                NotificationType::SomeoneSentMessage,
                *user_ids.choose(&mut rand::thread_rng()).unwrap(),
                Sentence(3..5).fake(),
            )
        })
        .collect::<Vec<_>>();
    for notification in sample_notifications {
        notification_collection
            .insert_one(notification)
            .await
            .unwrap();
    }

    println!("Sample data loaded successfully");
    Ok(())
}
