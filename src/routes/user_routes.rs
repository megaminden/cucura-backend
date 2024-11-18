use actix_web::{web, HttpResponse, Responder};
use futures::StreamExt;
use models::user::User;
use mongodb::{bson::doc, Client, Collection};

use crate::models;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users/register").route(web::post().to(register_user)))
        .service(web::resource("/users/update").route(web::put().to(update_user)))
        .service(web::resource("/users/delete/{username}").route(web::delete().to(delete_user)))
        .service(web::resource("/users").route(web::get().to(find_all_users)))
        .service(web::resource("/users/{username}").route(web::get().to(find_user)))
        .service(web::resource("/users/find/{username}").route(web::get().to(find_user)));
}

pub async fn register_user(client: web::Data<Client>, user: web::Json<User>) -> impl Responder {
    println!();
    let collection: Collection<User> = client.database("cucura-ccdb").collection("users");
    let new_user = User {
        id: user.id.clone(),
        username: user.username.clone(),
        email: user.email.clone(),
        password: user.password.clone(), // Note: Password should be hashed before storing
    };

    //check if user already exists
    let filter = doc! { "username": &new_user.username };
    let user_exists = collection.find_one(filter.clone()).await.unwrap();
    match user_exists {
        Some(_) => return HttpResponse::Ok().json("Error 10001 : User already exists"),
        None => (),
    }

    let insert_result = collection.insert_one(&new_user).await;

    match insert_result {
        Ok(_) => {
            client
                .database("cucura-ccdb")
                .collection("profiles")
                .insert_one(models::profile::Profile::new(
                    new_user.username.clone(),
                    new_user.email.clone(),
                ))
                .await
                .unwrap();
            HttpResponse::Ok().json("User registered successfully")
        }
        Err(e) => {
            eprintln!("Failed to insert document: {}", e);
            HttpResponse::InternalServerError().json("Failed to register user")
        }
    }
}

pub async fn update_user(client: web::Data<Client>, user: web::Json<User>) -> impl Responder {
    let collection: Collection<User> = client.database("cucura-ccdb").collection("users");
    let filter = doc! { "username": &user.username };
    let update = doc! { "$set": { "email": &user.email, "password": &user.password } }; // Note: Password should be hashed before storing

    let update_result = collection.update_one(filter, update).await;

    match update_result {
        Ok(_) => HttpResponse::Ok().json("User updated successfully"),
        Err(e) => {
            eprintln!("Failed to update document: {}", e);
            HttpResponse::InternalServerError().json("Failed to update user")
        }
    }
}

pub async fn delete_user(client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let collection: Collection<User> = client.database("cucura-ccdb").collection("users");
    let username = path.into_inner();
    let filter = doc! { "username": &username };

    let delete_result = collection.delete_one(filter).await;

    match delete_result {
        Ok(_) => HttpResponse::Ok().json("User deleted successfully"),
        Err(e) => {
            eprintln!("Failed to delete document: {}", e);
            HttpResponse::InternalServerError().json("Failed to delete user")
        }
    }
}
pub async fn find_user(client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let collection: Collection<User> = client.database("cucura-ccdb").collection("users");
    let username = path.into_inner();
    let filter = doc! { "username": &username };

    match collection.find_one(filter).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json("User not found"),
        Err(e) => {
            eprintln!("Failed to find document: {}", e);
            HttpResponse::InternalServerError().json("Failed to find user")
        }
    }
}

async fn find_all_users(client: web::Data<Client>) -> impl Responder {
    let collection: Collection<User> = client.database("cucura-ccdb").collection("users");
    // Create a filter (empty filter to match all documents)
    let filter = doc! {};

    // Find the documents in the collection matching the filter
    let mut cursor = collection.find(filter).await.unwrap();

    // Collect the documents into a vector
    let mut users = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(user) => users.push(user),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    // Return the documents as a JSON response
    HttpResponse::Ok().json(users)
}
