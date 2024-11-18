use actix_web::{web, HttpResponse, Responder};
use futures::StreamExt;
use models::profile::Profile;
use mongodb::{bson::doc, Client, Collection};

use crate::models;

pub fn profile_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/profiles/register").route(web::post().to(register_profile)))
        .service(web::resource("/profiles/update").route(web::put().to(update_profile)))
        .service(web::resource("/profiles/delete/{email}").route(web::delete().to(delete_profile)))
        .service(web::resource("/profiles").route(web::get().to(find_all_profiles)))
        .service(web::resource("/profiles/{email}").route(web::get().to(find_profile)))
        .service(web::resource("/profiles/find/{email}").route(web::get().to(find_profile)));
}

pub async fn register_profile(
    client: web::Data<Client>,
    profile: web::Json<Profile>,
) -> impl Responder {
    println!();
    let collection = client.database("cucura-ccdb").collection("profiles");
    let new_profile = Profile {
        id: profile.id.clone(),
        username: profile.username.clone(),
        email: profile.email.clone(),
        bio: profile.bio.clone(), // Note: Password should be hashed before storing
        business: profile.business.clone(),
        interests: profile.interests.clone(),
        profile_type: profile.profile_type.clone(),
    };

    let insert_result = collection.insert_one(new_profile).await;

    match insert_result {
        Ok(_) => HttpResponse::Ok().json("profile registered successfully"),
        Err(e) => {
            eprintln!("Failed to insert document: {}", e);
            HttpResponse::InternalServerError().json("Failed to register profile")
        }
    }
}

pub async fn update_profile(
    client: web::Data<Client>,
    profile: web::Json<Profile>,
) -> impl Responder {
    let collection: Collection<Profile> = client.database("cucura-ccdb").collection("profiles");
    let filter = doc! { "email": &profile.email };
    let update = doc! { "$set": { "username": &profile.username, "bio": &profile.bio } }; // Note: Password should be hashed before storing

    let update_result = collection.update_one(filter, update).await;

    match update_result {
        Ok(_) => HttpResponse::Ok().json("profile updated successfully"),
        Err(e) => {
            eprintln!("Failed to update document: {}", e);
            HttpResponse::InternalServerError().json("Failed to update profile")
        }
    }
}

pub async fn delete_profile(client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let collection: Collection<Profile> = client.database("cucura-ccdb").collection("profiles");
    let email = path.into_inner();
    let filter = doc! { "email": &email };

    let delete_result = collection.delete_one(filter).await;

    match delete_result {
        Ok(_) => HttpResponse::Ok().json("profile deleted successfully"),
        Err(e) => {
            eprintln!("Failed to delete document: {}", e);
            HttpResponse::InternalServerError().json("Failed to delete profile")
        }
    }
}
pub async fn find_profile(client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let collection: Collection<Profile> = client.database("cucura-ccdb").collection("profiles");
    let email = path.into_inner();
    let filter = doc! { "email": &email };

    match collection.find_one(filter).await {
        Ok(Some(profile)) => HttpResponse::Ok().json(profile),
        Ok(None) => HttpResponse::NotFound().json("profile not found"),
        Err(e) => {
            eprintln!("Failed to find document: {}", e);
            HttpResponse::InternalServerError().json("Failed to find profile")
        }
    }
}

async fn find_all_profiles(client: web::Data<Client>) -> impl Responder {
    let collection: Collection<Profile> = client.database("cucura-ccdb").collection("profiles");
    // Create a filter (empty filter to match all documents)
    let filter = doc! {};

    // Find the documents in the collection matching the filter
    let mut cursor = collection.find(filter).await.unwrap();

    // Collect the documents into a vector
    let mut profiles = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(profile) => profiles.push(profile),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    // Return the documents as a JSON response
    HttpResponse::Ok().json(profiles)
}
