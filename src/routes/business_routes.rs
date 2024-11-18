// routes/business_routes.rs
use actix_web::{web, HttpResponse, Responder};
use futures::StreamExt;
use models::business::Business;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection,
};

use crate::models;

pub fn business_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/businesses/register").route(web::post().to(register_business)))
        .service(web::resource("/businesses/update").route(web::put().to(update_business)))
        .service(web::resource("/businesses/delete/{id}").route(web::delete().to(delete_business)))
        .service(web::resource("/businesses").route(web::get().to(find_all_businesses)))
        .service(web::resource("/businesses/{id}").route(web::get().to(find_business)))
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
    let new_business = Business {
        id: business.id.clone(),
        user_id: business.user_id.clone(),
        name: business.name.clone(),
        description: business.description.clone(),
        logo: business.logo.clone(),
        pictures: business.pictures.clone(),
        founder: business.founder.clone(),
        industry: business.industry.clone(),
        phone: business.phone.clone(),
        address: business.address.clone(),
        city: business.city.clone(),
        region: business.region.clone(),
        country: business.country.clone(),
        website: business.website.clone(),
        contact_email: business.contact_email.clone(),
        created_at: business.created_at.clone(),
        updated_at: business.updated_at.clone(),
    };

    let insert_result = collection.insert_one(new_business).await;

    match insert_result {
        Ok(_) => HttpResponse::Ok().json("business registered successfully"),
        Err(e) => {
            eprintln!("Failed to insert document: {}", e);
            HttpResponse::InternalServerError().json("Failed to register business")
        }
    }
}

pub async fn update_business(
    client: web::Data<Client>,
    business: web::Json<Business>,
) -> impl Responder {
    let collection: Collection<Business> = client.database("cucura-ccdb").collection("businesses");
    let filter = doc! { "_id": &business.id };
    let update = doc! { "$set": {
        "user_id": &business.user_id,
        "name": &business.name,
        "description": &business.description,
        "logo": &business.logo,
        "pictures": &business.pictures,
        "founder": &business.founder,
        "industry": &business.industry,
        "phone": &business.phone,
        "address": &business.address,
        "city": &business.city,
        "region": &business.region,
        "country": &business.country,
        "website": &business.website,
        "contact_email": &business.contact_email,
        "created_at": &business.created_at,
        "updated_at": &business.updated_at
    }};

    let update_result = collection.update_one(filter, update).await;

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
    let id = path.into_inner();
    let filter = doc! { "_id": ObjectId::parse_str(&id).unwrap() };

    let delete_result = collection.delete_one(filter).await;

    match delete_result {
        Ok(_) => HttpResponse::Ok().json("business deleted successfully"),
        Err(e) => {
            eprintln!("Failed to delete document: {}", e);
            HttpResponse::InternalServerError().json("Failed to delete business")
        }
    }
}

pub async fn find_business(client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let collection: Collection<Business> = client.database("cucura-ccdb").collection("businesses");
    let id = path.into_inner();
    let filter = doc! { "_id": ObjectId::parse_str(&id).unwrap() };

    match collection.find_one(filter).await {
        Ok(Some(business)) => HttpResponse::Ok().json(business),
        Ok(None) => HttpResponse::NotFound().json("business not found"),
        Err(e) => {
            eprintln!("Failed to find document: {}", e);
            HttpResponse::InternalServerError().json("Failed to find business")
        }
    }
}

pub async fn find_all_businesses(client: web::Data<Client>) -> impl Responder {
    let collection: Collection<Business> = client.database("cucura-ccdb").collection("businesses");
    let filter = doc! {};

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

//create new function of find busineses by user_id
pub async fn find_businesses_by_user_id(
    client: web::Data<Client>,
    path: web::Path<String>,
) -> impl Responder {
    let collection: Collection<Business> = client.database("cucura-ccdb").collection("businesses");
    let user_id = path.into_inner();
    let filter = doc! { "user_id": &user_id };

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
