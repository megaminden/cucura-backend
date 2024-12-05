// routes/business_routes.rs
use actix_web::{web, HttpResponse, Responder};
use bson::serde_helpers::uuid_1_as_binary;
use futures::StreamExt;
use models::business::Business;
use mongodb::{
    bson::{self, doc, oid::ObjectId, Bson},
    Client, Collection,
};
use uuid::Uuid;

use crate::models;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

pub fn business_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/businesses/register").route(web::post().to(register_business)))
        .service(web::resource("/businesses/update").route(web::put().to(update_business)))
        .service(web::resource("/businesses/delete/{id}").route(web::delete().to(delete_business)))
        .service(web::resource("/businesses").route(web::get().to(find_all_businesses)))
        .service(web::resource("/businesses/{business_id}").route(web::get().to(find_business)))
        .service(
            web::resource("/businesses/user/{user_id}")
                .route(web::get().to(find_businesses_by_user_id)),
        );
}

pub async fn register_business(
    client: web::Data<Client>,
    business: web::Json<Business>,
) -> impl Responder {
    let collection = client.database("cucura-ccdb").collection("businesses");
    let new_business = business.into_inner();
    //find if business already exists
    let filter = doc! { "name": &new_business.name };
    let business_exists = collection.find_one(filter.clone()).await.unwrap();
    match business_exists {
        Some(_) => return HttpResponse::Ok().json("Error 10001 : Business already exists"),
        None => (),
    }

    let insert_result = collection.insert_one(new_business).await;

    match insert_result {
        Ok(_) => HttpResponse::Ok().json("business registered successfully"),
        Err(e) => {
            eprintln!("Failed to insert document: {}", e);
            HttpResponse::InternalServerError().json("Failed to register business")
        }
    }
}

pub async fn find_business(client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let collection: Collection<Business> = client.database("cucura-ccdb").collection("businesses");
    let business_id = path.into_inner();
    let business_id_result = Uuid::parse_str(&business_id);
    let business_id_uuid: Uuid;
    match business_id_result {
        Ok(uuid) => business_id_uuid = uuid,
        Err(_) => return HttpResponse::BadRequest().json("Invalid business ID format"),
    };
    let filter = doc! { "business_id": Bson::Binary(bson::Binary {
         subtype: bson::spec::BinarySubtype::UserDefined(0), bytes: business_id_uuid.as_bytes().to_vec() }) };
    match collection.find_one(filter).await {
        Ok(Some(business)) => HttpResponse::Ok().json(business),
        Ok(None) => HttpResponse::NotFound().json("Business not found"),
        Err(e) => {
            eprintln!("Failed to find document: {}", e);
            HttpResponse::InternalServerError().json("Failed to find business")
        }
    }
}

pub async fn update_business(
    client: web::Data<Client>,
    business: web::Json<Business>,
) -> impl Responder {
    let collection: Collection<Business> = client.database("cucura-ccdb").collection("businesses");
    let mut new_business = business.into_inner();
    //check if business already exists
    let business_id_result = Uuid::parse_str(&new_business.business_id.to_string());
    let business_id_uuid: Uuid;
    match business_id_result {
        Ok(uuid) => business_id_uuid = uuid,
        Err(_) => return HttpResponse::BadRequest().json("Invalid business ID format"),
    };

    let binary_business_id = Bson::Binary(bson::Binary {
        subtype: bson::spec::BinarySubtype::UserDefined(0),
        bytes: business_id_uuid.as_bytes().to_vec(),
    });
    let filter = doc! { "business_id": &binary_business_id};
    let business_exists = collection.find_one(filter.clone()).await.unwrap();
    match business_exists {
        Some(_) => (),
        None => return HttpResponse::Ok().json("Error 10001 : Business does not exist"),
    }

    //let update_doc = doc! { "$set": bson::to_document(&new_business).unwrap() };
    let update_doc = doc! {
        "$set": {
            "name": &new_business.name,
            "description": &new_business.description,
            "logo": &new_business.logo,
            "pictures": &new_business.pictures,
            "founder": &new_business.founder,
            "industry": &new_business.industry,
            "phone": &new_business.phone,
            "address": &new_business.address,
            "city": &new_business.city,
            "region": &new_business.region,
            "country": &new_business.country,
            "website": &new_business.website,
            "contact_email": &new_business.contact_email,
            "updated_at": &new_business.updated_at,
        }
    };
    let update_result = collection.update_one(filter, update_doc).await;

    match update_result {
        Ok(_) => HttpResponse::Ok().json("business updated successfully"),
        Err(e) => {
            eprintln!("Failed to update document: {}", e);
            HttpResponse::InternalServerError().json("Failed to update business")
        }
    }
}

pub async fn delete_business(client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let collection: Collection<Business> = client.database("cucura-ccdb").collection("businesses");
    let business_id = path.into_inner();
    let filter = doc! { "business_id": &business_id};

    let delete_result = collection.delete_one(filter).await;

    match delete_result {
        Ok(_) => HttpResponse::Ok().json("business deleted successfully"),
        Err(e) => {
            eprintln!("Failed to delete document: {}", e);
            HttpResponse::InternalServerError().json("Failed to delete business")
        }
    }
}

pub async fn find_all_businesses(
    client: web::Data<Client>,
    query: web::Query<PaginationParams>,
) -> impl Responder {
    let collection: Collection<Business> = client.database("cucura-ccdb").collection("businesses");
    let filter = doc! {};

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);
    let skip = (page - 1) * limit;

    let mut cursor = collection
        .find(filter)
        .await
        .unwrap()
        .skip(skip as usize)
        .take(limit as usize);

    let mut businesses = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(business) => businesses.push(business),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    HttpResponse::Ok().json(businesses)
}

//create new function of find busineses by user_id
pub async fn find_businesses_by_user_id(
    client: web::Data<Client>,
    path: web::Path<String>,
) -> impl Responder {
    let collection: Collection<Business> = client.database("cucura-ccdb").collection("businesses");
    let user_id = path.into_inner();
    //check if user exists
    let collection_user: Collection<models::user::User> =
        client.database("cucura-ccdb").collection("users");
    let filter_user = doc! {"user_id": &user_id};
    let user_exists = collection_user.find_one(filter_user.clone()).await.unwrap();
    match user_exists {
        Some(_) => (),
        None => return HttpResponse::Ok().json("Error 10001 : User does not exist"),
    }

    let filter = doc! { "user_ids": {"$elemMatch": &user_id} };

    let mut cursor = collection.find(filter).await.unwrap();

    let mut businesses = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(business) => businesses.push(business),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    HttpResponse::Ok().json(businesses)
}
