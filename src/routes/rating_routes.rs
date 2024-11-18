// routes/rating_routes.rs
use actix_web::{web, HttpResponse, Responder};
use futures::StreamExt;
use models::rating::Rating;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection,
};

use crate::models;

pub fn rating_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ratings/add").route(web::post().to(add_rating)))
        .service(web::resource("/ratings/update").route(web::put().to(update_rating)))
        .service(web::resource("/ratings/delete/{id}").route(web::delete().to(delete_rating)))
        .service(web::resource("/ratings").route(web::get().to(find_all_ratings)))
        .service(web::resource("/ratings/{id}").route(web::get().to(find_rating)));
}

pub async fn add_rating(client: web::Data<Client>, rating: web::Json<Rating>) -> impl Responder {
    let collection = client.database("cucura-ccdb").collection("ratings");
    let new_rating = Rating {
        user_id: rating.user_id.clone(),
        score: rating.score,
        comment: rating.comment.clone(),
        id: None,
    };

    let insert_result = collection.insert_one(new_rating).await;

    match insert_result {
        Ok(_) => HttpResponse::Ok().json("rating added successfully"),
        Err(e) => {
            eprintln!("Failed to insert document: {}", e);
            HttpResponse::InternalServerError().json("Failed to add rating")
        }
    }
}

pub async fn update_rating(client: web::Data<Client>, rating: web::Json<Rating>) -> impl Responder {
    let collection: Collection<Rating> = client.database("cucura-ccdb").collection("ratings");
    let filter = doc! { "user_id": &rating.user_id };
    let update = doc! { "$set": { "user_id": &rating.user_id, "score": &rating.score, "comment": &rating.comment } };

    let update_result = collection.update_one(filter, update).await;

    match update_result {
        Ok(_) => HttpResponse::Ok().json("rating updated successfully"),
        Err(e) => {
            eprintln!("Failed to update document: {}", e);
            HttpResponse::InternalServerError().json("Failed to update rating")
        }
    }
}

pub async fn delete_rating(client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let collection: Collection<Rating> = client.database("cucura-ccdb").collection("ratings");
    let id = path.into_inner();
    let filter = doc! { "_id": ObjectId::parse_str(&id).unwrap() };

    let delete_result = collection.delete_one(filter).await;

    match delete_result {
        Ok(_) => HttpResponse::Ok().json("rating deleted successfully"),
        Err(e) => {
            eprintln!("Failed to delete document: {}", e);
            HttpResponse::InternalServerError().json("Failed to delete rating")
        }
    }
}

pub async fn find_rating(client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let collection: Collection<Rating> = client.database("cucura-ccdb").collection("ratings");
    let id = path.into_inner();
    let filter = doc! { "_id": ObjectId::parse_str(&id).unwrap() };

    match collection.find_one(filter).await {
        Ok(Some(rating)) => HttpResponse::Ok().json(rating),
        Ok(None) => HttpResponse::NotFound().json("rating not found"),
        Err(e) => {
            eprintln!("Failed to find document: {}", e);
            HttpResponse::InternalServerError().json("Failed to find rating")
        }
    }
}

pub async fn find_all_ratings(client: web::Data<Client>) -> impl Responder {
    let collection: Collection<Rating> = client.database("cucura-ccdb").collection("ratings");
    let filter = doc! {};

    let mut cursor = collection.find(filter).await.unwrap();

    let mut ratings = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(rating) => ratings.push(rating),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    HttpResponse::Ok().json(ratings)
}
