// routes/payment_routes.rs
use actix_web::{web, HttpResponse, Responder};
use futures::StreamExt;
use models::payment::Payment;
use mongodb::{
    bson::{self, doc},
    Client, Collection,
};

use crate::models;

pub fn payment_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/payments/add").route(web::post().to(add_payment)))
        .service(web::resource("/payments/update").route(web::put().to(update_payment)))
        .service(web::resource("/payments/delete/{id}").route(web::delete().to(delete_payment)))
        .service(web::resource("/payments").route(web::get().to(find_all_payments)))
        .service(web::resource("/payments/{id}").route(web::get().to(find_payment)))
        .service(
            web::resource("/payments/seller/{seller_id}")
                .route(web::get().to(find_payments_by_seller_id)),
        )
        .service(
            web::resource("/payments/purchaser/{purchaser_id}")
                .route(web::get().to(find_payments_by_purchaser_id)),
        );
}

pub async fn add_payment(client: web::Data<Client>, payment: web::Json<Payment>) -> impl Responder {
    let collection = client.database("cucura-ccdb").collection("payments");
    let new_payment = payment.into_inner();

    let insert_result = collection.insert_one(new_payment).await;

    match insert_result {
        Ok(_) => HttpResponse::Ok().json("payment added successfully"),
        Err(e) => {
            eprintln!("Failed to insert document: {}", e);
            HttpResponse::InternalServerError().json("Failed to add payment")
        }
    }
}

pub async fn update_payment(
    client: web::Data<Client>,
    payment: web::Json<Payment>,
) -> impl Responder {
    let collection: Collection<Payment> = client.database("cucura-ccdb").collection("payments");
    let payment_for_update = payment.into_inner();
    //check if payment already exists
    let filter = doc! {"payment_id": payment_for_update.payment_id.to_string() };
    let payment_exists = collection.find_one(filter.clone()).await.unwrap();
    match payment_exists {
        Some(_) => (),
        None => return HttpResponse::Ok().json("Error 10001 : Payment does not exist"),
    }
    let update_doc = doc! { "$set": bson::to_document(&payment_for_update).unwrap() };
    let update_result = collection.update_one(filter, update_doc).await;

    match update_result {
        Ok(_) => HttpResponse::Ok().json("payment updated successfully"),
        Err(e) => {
            eprintln!("Failed to update document: {}", e);
            HttpResponse::InternalServerError().json("Failed to update payment")
        }
    }
}

pub async fn delete_payment(client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let collection: Collection<Payment> = client.database("cucura-ccdb").collection("payments");
    let payment_id = path.into_inner();
    let filter = doc! { "payment_id": &payment_id};

    let delete_result = collection.delete_one(filter).await;

    match delete_result {
        Ok(_) => HttpResponse::Ok().json(format!("Payment {} : deleted successfully", &payment_id)),
        Err(e) => {
            eprintln!("Failed to delete document: {}", e);
            HttpResponse::InternalServerError().json("Failed to delete payment")
        }
    }
}

pub async fn find_payment(client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let collection: Collection<Payment> = client.database("cucura-ccdb").collection("payments");
    let payment_id = path.into_inner();
    let filter = doc! { "payment_id": payment_id};

    match collection.find_one(filter).await {
        Ok(Some(payment)) => HttpResponse::Ok().json(payment),
        Ok(None) => HttpResponse::NotFound().json("payment not found"),
        Err(e) => {
            eprintln!("Failed to find document: {}", e);
            HttpResponse::InternalServerError().json("Failed to find payment")
        }
    }
}

pub async fn find_all_payments(client: web::Data<Client>) -> impl Responder {
    let collection: Collection<Payment> = client.database("cucura-ccdb").collection("payments");
    let filter = doc! {};

    let mut cursor = collection.find(filter).await.unwrap();

    let mut payments = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(payment) => payments.push(payment),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    HttpResponse::Ok().json(payments)
}

pub async fn find_payments_by_seller_id(
    client: web::Data<Client>,
    path: web::Path<String>,
) -> impl Responder {
    let collection: Collection<Payment> = client.database("cucura-ccdb").collection("payments");
    let seller_id = path.into_inner();
    let filter = doc! { "seller_id": seller_id };

    let mut cursor = collection.find(filter).await.unwrap();

    let mut payments = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(payment) => payments.push(payment),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    HttpResponse::Ok().json(payments)
}

pub async fn find_payments_by_purchaser_id(
    client: web::Data<Client>,
    path: web::Path<String>,
) -> impl Responder {
    let collection: Collection<Payment> = client.database("cucura-ccdb").collection("payments");
    let purchaser_id = path.into_inner();
    let filter = doc! { "purchaser_id": purchaser_id };

    let mut cursor = collection.find(filter).await.unwrap();

    let mut payments = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(payment) => payments.push(payment),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    HttpResponse::Ok().json(payments)
}
