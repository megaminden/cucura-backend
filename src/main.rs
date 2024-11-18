mod models;
mod routes;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use mongodb::{
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};
use routes::{profile_routes, rating_routes, review_routes, user_routes};

use std::env;

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

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
        App::new()
            .app_data(web::Data::new(client.clone()))
            .configure(user_routes::user_routes)
            .configure(rating_routes::rating_routes)
            .configure(profile_routes::profile_routes)
            .configure(routes::review_routes::review_routes)
            .route("/", web::get().to(greet))
    })
    .bind(("0.0.0.0", 5001))?
    .run()
    .await

    //println!("Server running at http://localhost:5001/");

    //server.await
}
