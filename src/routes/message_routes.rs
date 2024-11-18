use actix_web::{web, HttpResponse, Responder};

use crate::models::message::Message;
use crate::models::notification::Notification;
use futures::stream::StreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, Client, Collection};

pub fn message_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/messages/send").route(web::post().to(send_message)))
        .service(web::resource("/messages/delete/{id}").route(web::delete().to(delete_message)))
        .service(web::resource("/messages").route(web::get().to(find_all_messages)))
        .service(web::resource("/messages/{id}").route(web::get().to(find_message)))
        .service(web::resource("/notifications/create").route(web::post().to(create_notification)))
        .service(web::resource("/notifications/{user_id}").route(web::get().to(get_notifications)))
        .service(
            web::resource("/notifications/confirm/{id}").route(web::put().to(confirm_notification)),
        )
        .service(
            web::resource("/notifications/delete/{id}")
                .route(web::delete().to(delete_notification)),
        );
}

async fn send_message(client: web::Data<Client>, message: web::Json<Message>) -> impl Responder {
    let collection = client.database("cucura-ccdb").collection("messages");
    let new_message = Message {
        id: message.id.clone(),
        sender: message.sender.clone(),
        receiver: message.receiver.clone(),
        content: message.content.clone(),
        timestamp: message.timestamp.clone(),
    };

    let insert_result = collection.insert_one(new_message).await;

    match insert_result {
        Ok(_) => HttpResponse::Ok().json("Message sent successfully"),
        Err(e) => {
            eprintln!("Failed to insert document: {}", e);
            HttpResponse::InternalServerError().json("Failed to send message")
        }
    }
}

pub async fn find_message(client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let collection: Collection<Message> = client.database("cucura-ccdb").collection("messages");
    let id = path.into_inner();
    let filter = doc! { "_id": ObjectId::parse_str(&id).unwrap() };

    match collection.find_one(filter).await {
        Ok(Some(message)) => HttpResponse::Ok().json(message),
        Ok(None) => HttpResponse::NotFound().json("Message not found"),
        Err(e) => {
            eprintln!("Failed to find document: {}", e);
            HttpResponse::InternalServerError().json("Failed to find message")
        }
    }
}

pub async fn delete_message(client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let collection: Collection<Message> = client.database("cucura-ccdb").collection("messages");
    let id = path.into_inner();
    let filter = doc! { "_id": ObjectId::parse_str(&id).unwrap() };

    let delete_result = collection.delete_one(filter).await;

    match delete_result {
        Ok(_) => HttpResponse::Ok().json("Message deleted successfully"),
        Err(e) => {
            eprintln!("Failed to delete document: {}", e);
            HttpResponse::InternalServerError().json("Failed to delete message")
        }
    }
}

pub async fn find_all_messages(client: web::Data<Client>) -> impl Responder {
    let collection: Collection<Message> = client.database("cucura-ccdb").collection("messages");
    let filter = doc! {};

    let mut cursor = collection.find(filter).await.unwrap();

    let mut messages = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(message) => messages.push(message),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    HttpResponse::Ok().json(messages)
}

pub async fn create_notification(
    client: web::Data<Client>,
    notification: web::Json<Notification>,
) -> impl Responder {
    let collection = client.database("cucura-ccdb").collection("notifications");
    let new_notification = Notification {
        id: notification.id.clone(),
        user_id: notification.user_id.clone(),
        message: notification.message.clone(),
        confirmed: notification.confirmed,
        timestamp: notification.timestamp.clone(),
    };

    let insert_result = collection.insert_one(new_notification).await;

    match insert_result {
        Ok(_) => HttpResponse::Ok().json("Notification created successfully"),
        Err(e) => {
            eprintln!("Failed to insert document: {}", e);
            HttpResponse::InternalServerError().json("Failed to create notification")
        }
    }
}

async fn get_notifications(
    client: web::Data<Client>,
    user_id: web::Path<String>,
) -> impl Responder {
    let collection = client.database("cucura-ccdb").collection("notifications");
    let filter = doc! { "user_id": user_id.into_inner() };

    let cursor = collection.find(filter).await;

    match cursor {
        Ok(mut cursor) => {
            let mut notifications = Vec::new();
            while let Some(result) = cursor.next().await {
                match result {
                    Ok(document) => {
                        if let Ok(notification) =
                            mongodb::bson::from_document::<Notification>(document)
                        {
                            notifications.push(notification);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to parse document: {}", e);
                        return HttpResponse::InternalServerError()
                            .json("Failed to parse notifications");
                    }
                }
            }
            HttpResponse::Ok().json(notifications)
        }
        Err(e) => {
            eprintln!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().json("Failed to get notifications")
        }
    }
}

pub async fn confirm_notification(
    client: web::Data<Client>,
    path: web::Path<String>,
) -> impl Responder {
    let collection: Collection<Notification> =
        client.database("cucura-ccdb").collection("notifications");
    let id = path.into_inner();
    let filter = doc! { "_id": ObjectId::parse_str(&id).unwrap() };
    let update = doc! { "$set": { "confirmed": true } };

    let update_result = collection.update_one(filter, update).await;

    match update_result {
        Ok(_) => HttpResponse::Ok().json("Notification confirmed successfully"),
        Err(e) => {
            eprintln!("Failed to update document: {}", e);
            HttpResponse::InternalServerError().json("Failed to confirm notification")
        }
    }
}

pub async fn delete_notification(
    client: web::Data<Client>,
    path: web::Path<String>,
) -> impl Responder {
    let collection: Collection<Notification> =
        client.database("cucura-ccdb").collection("notifications");
    let id = path.into_inner();
    let filter = doc! { "_id": ObjectId::parse_str(&id).unwrap() };

    let delete_result = collection.delete_one(filter).await;

    match delete_result {
        Ok(_) => HttpResponse::Ok().json("Notification deleted successfully"),
        Err(e) => {
            eprintln!("Failed to delete document: {}", e);
            HttpResponse::InternalServerError().json("Failed to delete notification")
        }
    }
}
