// routes/payment_routes.rs
use actix_web::{web, HttpResponse, Responder};
use futures::StreamExt;
use models::payment::Payment;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, Client, Collection};

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
    let new_payment = Payment {
        id: None,
        purchaser_id: payment.purchaser_id.clone(),
        seller_id: payment.seller_id.clone(),
        payment_type: payment.payment_type.clone(),
        description: payment.description.clone(),
        amount: payment.amount,
        currency: payment.currency.clone(),
        status: payment.status.clone(),
        created_at: payment.created_at.clone(),
        updated_at: payment.updated_at.clone(),
    };

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
    let filter = doc! { "_id": &payment.id };
    let update = doc! { "$set": {
        "purchaser_id": &payment.purchaser_id,
        "seller_id": &payment.seller_id,
        "payment_type": &payment.payment_type,
        "description": &payment.description,
        "amount": &payment.amount,
        "currency": &payment.currency,
        "status": &payment.status,
        "created_at": &payment.created_at,
        "updated_at": &payment.updated_at
    }};

    let update_result = collection.update_one(filter, update).await;

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
    let id = path.into_inner();
    let filter = doc! { "_id": ObjectId::parse_str(&id).unwrap() };

    let delete_result = collection.delete_one(filter).await;

    match delete_result {
        Ok(_) => HttpResponse::Ok().json("payment deleted successfully"),
        Err(e) => {
            eprintln!("Failed to delete document: {}", e);
            HttpResponse::InternalServerError().json("Failed to delete payment")
        }
    }
}

pub async fn find_payment(client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let collection: Collection<Payment> = client.database("cucura-ccdb").collection("payments");
    let id = path.into_inner();
    let filter = doc! { "_id": ObjectId::parse_str(&id).unwrap() };

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
