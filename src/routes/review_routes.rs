// routes/review_routes.rs
use actix_web::{web, HttpResponse, Responder};
use futures::StreamExt;
use models::review::Review;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, Client, Collection};

use crate::models;

pub fn review_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/reviews/add").route(web::post().to(add_review)))
        .service(web::resource("/reviews/update").route(web::put().to(update_review)))
        .service(web::resource("/reviews/delete/{id}").route(web::delete().to(delete_review)))
        .service(web::resource("/reviews").route(web::get().to(find_all_reviews)))
        .service(web::resource("/reviews/{id}").route(web::get().to(find_review)));
}

pub async fn add_review(client: web::Data<Client>, review: web::Json<Review>) -> impl Responder {
    let collection = client.database("cucura-ccdb").collection("reviews");
    let new_review = Review {
        user_id: review.user_id.clone(),
        product_id: review.product_id.clone(),
        rating: review.rating,
        comment: review.comment.clone(),
        id: None,
    };

    let insert_result = collection.insert_one(new_review).await;

    match insert_result {
        Ok(_) => HttpResponse::Ok().json("review added successfully"),
        Err(e) => {
            eprintln!("Failed to insert document: {}", e);
            HttpResponse::InternalServerError().json("Failed to add review")
        }
    }
}

pub async fn update_review(client: web::Data<Client>, review: web::Json<Review>) -> impl Responder {
    let collection: Collection<Review> = client.database("cucura-ccdb").collection("reviews");
    let filter = doc! { "_id": &review.id };
    let update = doc! { "$set": { "user_id": &review.user_id, "product_id": &review.product_id, "rating": &review.rating, "comment": &review.comment } };

    let update_result = collection.update_one(filter, update).await;

    match update_result {
        Ok(_) => HttpResponse::Ok().json("review updated successfully"),
        Err(e) => {
            eprintln!("Failed to update document: {}", e);
            HttpResponse::InternalServerError().json("Failed to update review")
        }
    }
}

pub async fn delete_review(client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let collection: Collection<Review> = client.database("cucura-ccdb").collection("reviews");
    let id = path.into_inner();
    let filter = doc! { "_id": ObjectId::parse_str(&id).unwrap() };

    let delete_result = collection.delete_one(filter).await;

    match delete_result {
        Ok(_) => HttpResponse::Ok().json("review deleted successfully"),
        Err(e) => {
            eprintln!("Failed to delete document: {}", e);
            HttpResponse::InternalServerError().json("Failed to delete review")
        }
    }
}

pub async fn find_review(client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let collection: Collection<Review> = client.database("cucura-ccdb").collection("reviews");
    let id = path.into_inner();
    let filter = doc! { "_id": ObjectId::parse_str(&id).unwrap() };

    match collection.find_one(filter).await {
        Ok(Some(review)) => HttpResponse::Ok().json(review),
        Ok(None) => HttpResponse::NotFound().json("review not found"),
        Err(e) => {
            eprintln!("Failed to find document: {}", e);
            HttpResponse::InternalServerError().json("Failed to find review")
        }
    }
}

pub async fn find_all_reviews(client: web::Data<Client>) -> impl Responder {
    let collection: Collection<Review> = client.database("cucura-ccdb").collection("reviews");
    let filter = doc! {};

    let mut cursor = collection.find(filter).await.unwrap();

    let mut reviews = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(review) => reviews.push(review),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    HttpResponse::Ok().json(reviews)
}
