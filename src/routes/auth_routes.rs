// routes/auth_routes.rs
use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use futures::StreamExt;
use models::user::User;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection,
};
use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordChangeRequest {
    pub email: String,
    pub old_password: String,
    pub new_password: String,
}

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/auth/login").route(web::post().to(login_user)))
        .service(web::resource("/auth/set_password").route(web::post().to(set_password)))
        .service(web::resource("/auth/change_password").route(web::put().to(change_password)));
}

pub async fn login_user(
    client: web::Data<Client>,
    login_request: web::Json<LoginRequest>,
) -> impl Responder {
    let collection: Collection<User> = client.database("cucura-ccdb").collection("users");
    let filter = doc! { "username": &login_request.username };

    match collection.find_one(filter).await {
        Ok(Some(user)) => {
            if verify(&login_request.password, &user.password).unwrap() {
                HttpResponse::Ok().json("Login successful")
            } else {
                HttpResponse::Unauthorized().json("Invalid username or password")
            }
        }
        Ok(None) => HttpResponse::Unauthorized().json("Invalid username or password"),
        Err(e) => {
            eprintln!("Failed to find document: {}", e);
            HttpResponse::InternalServerError().json("Failed to login")
        }
    }
}

pub async fn set_password(client: web::Data<Client>, user: web::Json<User>) -> impl Responder {
    let collection: Collection<User> = client.database("cucura-ccdb").collection("users");
    let filter = doc! { "email": &user.email };
    let hashed_password = hash(&user.password, DEFAULT_COST).unwrap();
    let update = doc! { "$set": { "password": hashed_password } };

    match collection.update_one(filter, update).await {
        Ok(_) => HttpResponse::Ok().json("Password set successfully"),
        Err(e) => {
            eprintln!("Failed to update document: {}", e);
            HttpResponse::InternalServerError().json("Failed to set password")
        }
    }
}

pub async fn change_password(
    client: web::Data<Client>,
    password_change_request: web::Json<PasswordChangeRequest>,
) -> impl Responder {
    let collection: Collection<User> = client.database("cucura-ccdb").collection("users");
    let filter = doc! { "email": &password_change_request.email };

    match collection.find_one(filter.clone()).await {
        Ok(Some(user)) => {
            if verify(&password_change_request.old_password, &user.password).unwrap() {
                let hashed_password =
                    hash(&password_change_request.new_password, DEFAULT_COST).unwrap();
                let update = doc! { "$set": { "password": hashed_password } };

                match collection.update_one(filter, update).await {
                    Ok(_) => HttpResponse::Ok().json("Password changed successfully"),
                    Err(e) => {
                        eprintln!("Failed to update document: {}", e);
                        HttpResponse::InternalServerError().json("Failed to change password")
                    }
                }
            } else {
                HttpResponse::Unauthorized().json("Invalid old password")
            }
        }
        Ok(None) => HttpResponse::Unauthorized().json("Invalid username"),
        Err(e) => {
            eprintln!("Failed to find document: {}", e);
            HttpResponse::InternalServerError().json("Failed to change password")
        }
    }
}
