mod models;
mod routes;
use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use mongodb::{
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};
use routes::{
    auth_routes, business_review_routes, business_routes, message_routes, payment_routes,
    profile_routes, training_routes, user_routes,
};

use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv().ok();

    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    let mut client_options = ClientOptions::parse(&uri).await.unwrap();

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options).unwrap();

    println!("Starting web server...");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(client.clone()))
            .wrap(cors)
            .configure(user_routes::user_routes)
            .configure(profile_routes::profile_routes)
            .configure(business_review_routes::review_routes)
            .configure(training_routes::training_routes)
            .configure(business_routes::business_routes)
            .configure(payment_routes::payment_routes)
            .configure(message_routes::message_routes)
            .configure(auth_routes::auth_routes)
      
    })
    .bind(("0.0.0.0", 5001))?
    .run()
    .await

    //println!("Server running at http://localhost:5001/");

    //server.await
}
