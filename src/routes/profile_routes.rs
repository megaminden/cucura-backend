use actix_web::{web, HttpResponse, Responder};
use futures::StreamExt;
use models::profile::Profile;
use mongodb::{
    bson::{self, doc, oid::ObjectId, Bson},
    Client, Collection,
};
use uuid::Uuid;

use crate::models;

pub fn profile_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/profiles/register").route(web::post().to(register_profile)))
        .service(web::resource("/profiles/update").route(web::put().to(update_profile)))
        .service(
            web::resource("/profiles/delete/{username}").route(web::delete().to(delete_profile)),
        )
        .service(web::resource("/profiles").route(web::get().to(find_all_profiles)))
        .service(web::resource("/profiles/{profile_id}").route(web::get().to(find_profile)))
        .service(
            web::resource("/profiles/username/{username}")
                .route(web::get().to(find_profile_by_username)),
        );
}

pub async fn register_profile(
    client: web::Data<Client>,
    profile: web::Json<Profile>,
) -> impl Responder {
    println!();
    let collection = client.database("cucura-ccdb").collection("profiles");
    let new_profile = profile.into_inner();
    let filter = doc! { "email": &new_profile.email };
    let profile_exists = collection.find_one(filter.clone()).await.unwrap();
    match profile_exists {
        Some(_) => return HttpResponse::Ok().json("Error 10001 : Profile already exists"),
        None => (),
    }

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
    let new_profile = profile.into_inner();
    let filter = doc! { "username": &new_profile.username };

    let update = doc! { "$set": { "username": &new_profile.username, "bio": &new_profile.bio } }; // Note: Password should be hashed before storing

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
    let username = path.into_inner();
    let filter = doc! { "username": &username };

    let delete_result = collection.delete_one(filter).await;

    match delete_result {
        Ok(_) => HttpResponse::Ok().json("Profile deleted successfully"),
        Err(e) => {
            eprintln!("Failed to delete document: {}", e);
            HttpResponse::InternalServerError().json("Failed to delete profile")
        }
    }
}
pub async fn find_profile(client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let collection: Collection<Profile> = client.database("cucura-ccdb").collection("profiles");
    let profile_id = path.into_inner();
    let profile_uuid_result = Uuid::parse_str(&profile_id);
    let profile_uuid: Uuid;
    match profile_uuid_result {
        Ok(uuid) => profile_uuid = uuid,
        Err(_) => return HttpResponse::BadRequest().json("Invalid business ID format"),
    };
    let filter = doc! { "profile_id": Bson::Binary(bson::Binary {
    subtype: bson::spec::BinarySubtype::UserDefined(0), bytes: profile_uuid.as_bytes().to_vec() }) };
    match collection.find_one(filter).await {
        Ok(Some(profile)) => HttpResponse::Ok().json(profile),
        Ok(None) => HttpResponse::NotFound().json("Profile not found"),
        Err(e) => {
            eprintln!("Failed to find document: {}", e);
            HttpResponse::InternalServerError().json("Failed to find profile")
        }
    }
}

pub async fn find_profile_by_username(
    client: web::Data<Client>,
    path: web::Path<String>,
) -> impl Responder {
    let collection: Collection<Profile> = client.database("cucura-ccdb").collection("profiles");
    let username = path.into_inner();
    let filter = doc! { "username": &username };

    match collection.find_one(filter).await {
        Ok(Some(profile)) => HttpResponse::Ok().json(profile),
        Ok(None) => HttpResponse::NotFound().json("Profile not found"),
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
