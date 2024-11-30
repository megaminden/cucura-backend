use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};
use futures::StreamExt;
use models::user::User;
use mongodb::{bson::doc, Client, Collection};
use uuid::Uuid;

use crate::models::{self, user::NewUser};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users/register").route(web::post().to(register_user)))
        .service(web::resource("/users/update").route(web::put().to(update_user)))
        .service(web::resource("/users/delete/{user_id}").route(web::delete().to(delete_user)))
        .service(web::resource("/users").route(web::get().to(find_all_users)))
        .service(web::resource("/users/{username}").route(web::get().to(find_user)));
}

pub async fn register_user(
    client: web::Data<Client>,
    new_user: web::Json<NewUser>,
) -> impl Responder {
    let collection: Collection<User> = client.database("cucura-ccdb").collection("users");
    let mut new_user = new_user.into_inner();
    let hashed_password = hash(&new_user.password, DEFAULT_COST).unwrap();
    new_user.password = hashed_password;

    // Check if user already exists
    let filter = doc! { "username": &new_user.username };
    let user_exists = collection.find_one(filter.clone()).await.unwrap();
    match user_exists {
        Some(_) => HttpResponse::Ok().json("Error: User already exists"),
        None => {
            let created_user = new_user.to_user();
            collection.insert_one(&created_user).await.unwrap();
            //also insert into the user profile collection
            let profile_collection: Collection<models::profile::Profile> =
                client.database("cucura-ccdb").collection("profiles");
            let new_profile = models::profile::Profile::new(
                created_user.user_id,
                created_user.email.clone(),
                created_user.username.clone(),
            );
            profile_collection.insert_one(&new_profile).await.unwrap();

            HttpResponse::Ok().json("User registered successfully")
        }
    }
}

pub async fn update_user(client: web::Data<Client>, user: web::Json<User>) -> impl Responder {
    let collection: Collection<User> = client.database("cucura-ccdb").collection("users");
    let hashed_password = hash(&user.password, DEFAULT_COST).unwrap();
    let mut updated_user = user.into_inner();
    updated_user.password = hashed_password;

    let filter = doc! { "user_id": updated_user.user_id.to_string() };
    let update = doc! {
        "$set": {
            "username": updated_user.username,
            "email": updated_user.email,
            "password": updated_user.password,
            "user_type": updated_user.user_type,
        }
    };

    match collection.update_one(filter, update).await {
        Ok(_) => HttpResponse::Ok().json("User updated successfully"),
        Err(e) => {
            eprintln!("Failed to update document: {}", e);
            HttpResponse::InternalServerError().json("Failed to update user")
        }
    }
}

pub async fn delete_user(client: web::Data<Client>, path: web::Path<Uuid>) -> impl Responder {
    let collection: Collection<User> = client.database("cucura-ccdb").collection("users");
    let filter = doc! { "user_id": path.into_inner().to_string() };

    match collection.delete_one(filter).await {
        Ok(_) => HttpResponse::Ok().json("User deleted successfully"),
        Err(e) => {
            eprintln!("Failed to delete document: {}", e);
            HttpResponse::InternalServerError().json("Failed to delete user")
        }
    }
}

pub async fn find_user(client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let collection: Collection<User> = client.database("cucura-ccdb").collection("users");
    let filter = doc! { "username": path.into_inner() };

    match collection.find_one(filter).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json("User not found"),
        Err(e) => {
            eprintln!("Failed to find document: {}", e);
            HttpResponse::InternalServerError().json("Failed to find user")
        }
    }
}

pub async fn find_all_users(client: web::Data<Client>) -> impl Responder {
    let collection: Collection<User> = client.database("cucura-ccdb").collection("users");
    let filter = doc! {};

    let mut cursor = collection.find(filter).await.unwrap();
    let mut users = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(user) => users.push(user),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    HttpResponse::Ok().json(users)
}
