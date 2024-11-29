use actix_web::{web, HttpResponse, Responder};
use futures::StreamExt;
use mongodb::bson;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, Client, Collection};

use crate::models::business_review::BusinessReview;

pub fn review_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/reviews/add").route(web::post().to(add_review)))
        .service(web::resource("/reviews/update").route(web::put().to(update_review)))
        .service(web::resource("/reviews/delete/{id}").route(web::delete().to(delete_review)))
        .service(web::resource("/reviews").route(web::get().to(find_all_reviews)))
        .service(web::resource("/reviews/{id}").route(web::get().to(find_review)));
}

pub async fn add_review(
    client: web::Data<Client>,
    review: web::Json<BusinessReview>,
) -> impl Responder {
    let collection = client.database("cucura-ccdb").collection("reviews");
    let new_review = review.into_inner();
    let insert_result = collection.insert_one(new_review).await;

    match insert_result {
        Ok(_) => HttpResponse::Ok().json("review added successfully"),
        Err(e) => {
            eprintln!("Failed to insert document: {}", e);
            HttpResponse::InternalServerError().json("Failed to add review")
        }
    }
}

pub async fn update_review(
    client: web::Data<Client>,
    business_review: web::Json<BusinessReview>,
) -> impl Responder {
    let collection: Collection<BusinessReview> =
        client.database("cucura-ccdb").collection("reviews");
    let new_business_review = business_review.into_inner();
    //check if review already exists
    let business_review = bson::to_document(&new_business_review).unwrap();
    let filter = doc! { "business_review": &business_review };
    let review_exists = collection.find_one(filter.clone()).await.unwrap();
    match review_exists {
        Some(_) => (),
        None => return HttpResponse::Ok().json("Error 10001 : Review does not exist"),
    }
    let update_result = collection.update_one(filter, business_review).await;

    match update_result {
        Ok(_) => HttpResponse::Ok().json("review updated successfully"),
        Err(e) => {
            eprintln!("Failed to update document: {}", e);
            HttpResponse::InternalServerError().json("Failed to update review")
        }
    }
}

pub async fn delete_review(client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let collection: Collection<BusinessReview> =
        client.database("cucura-ccdb").collection("reviews");
    let business_review_id = path.into_inner();
    let filter = doc! { "business_review_id": business_review_id };

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
    let collection: Collection<BusinessReview> =
        client.database("cucura-ccdb").collection("reviews");
    let business_review_id = path.into_inner();
    let filter = doc! { "business_review_id": business_review_id };

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
    let collection: Collection<BusinessReview> =
        client.database("cucura-ccdb").collection("reviews");
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
