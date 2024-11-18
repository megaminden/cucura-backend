// routes/training_routes.rs
use actix_web::{web, HttpResponse, Responder};
use futures::StreamExt;
use models::training::Training;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection,
};

use crate::models;

pub fn training_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/trainings/add").route(web::post().to(add_training)))
        .service(web::resource("/trainings/update").route(web::put().to(update_training)))
        .service(web::resource("/trainings/delete/{id}").route(web::delete().to(delete_training)))
        .service(web::resource("/trainings").route(web::get().to(find_all_trainings)))
        .service(web::resource("/trainings/{id}").route(web::get().to(find_training)));
}

pub async fn add_training(
    client: web::Data<Client>,
    training: web::Json<Training>,
) -> impl Responder {
    let collection = client.database("cucura-ccdb").collection("trainings");
    let new_training = Training {
        id: None,
        title: training.title.clone(),
        description: training.description.clone(),
        date: training.date.clone(),
        duration: training.duration.clone(),
        trainer: training.trainer.clone(),
    };

    let insert_result = collection.insert_one(new_training).await;

    match insert_result {
        Ok(_) => HttpResponse::Ok().json("training added successfully"),
        Err(e) => {
            eprintln!("Failed to insert document: {}", e);
            HttpResponse::InternalServerError().json("Failed to add training")
        }
    }
}

pub async fn update_training(
    client: web::Data<Client>,
    training: web::Json<Training>,
) -> impl Responder {
    let collection: Collection<Training> = client.database("cucura-ccdb").collection("trainings");
    let filter = doc! { "_id": &training.id };
    let update = doc! { "$set": { "title": &training.title, "description": &training.description, "date": &training.date, "duration": &training.duration, "trainer": &training.trainer } };

    let update_result = collection.update_one(filter, update).await;

    match update_result {
        Ok(_) => HttpResponse::Ok().json("training updated successfully"),
        Err(e) => {
            eprintln!("Failed to update document: {}", e);
            HttpResponse::InternalServerError().json("Failed to update training")
        }
    }
}

pub async fn delete_training(client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let collection: Collection<Training> = client.database("cucura-ccdb").collection("trainings");
    let id = path.into_inner();
    let filter = doc! { "_id": ObjectId::parse_str(&id).unwrap() };

    let delete_result = collection.delete_one(filter).await;

    match delete_result {
        Ok(_) => HttpResponse::Ok().json("training deleted successfully"),
        Err(e) => {
            eprintln!("Failed to delete document: {}", e);
            HttpResponse::InternalServerError().json("Failed to delete training")
        }
    }
}

pub async fn find_training(client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let collection: Collection<Training> = client.database("cucura-ccdb").collection("trainings");
    let id = path.into_inner();
    let filter = doc! { "_id": ObjectId::parse_str(&id).unwrap() };

    match collection.find_one(filter).await {
        Ok(Some(training)) => HttpResponse::Ok().json(training),
        Ok(None) => HttpResponse::NotFound().json("training not found"),
        Err(e) => {
            eprintln!("Failed to find document: {}", e);
            HttpResponse::InternalServerError().json("Failed to find training")
        }
    }
}

pub async fn find_all_trainings(client: web::Data<Client>) -> impl Responder {
    let collection: Collection<Training> = client.database("cucura-ccdb").collection("trainings");
    let filter = doc! {};

    let mut cursor = collection.find(filter).await.unwrap();

    let mut trainings = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(training) => trainings.push(training),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    HttpResponse::Ok().json(trainings)
}
