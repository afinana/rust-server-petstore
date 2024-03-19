// main.rs

use actix_web::{web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod db;
mod handlers;
mod petmodel;
mod usermodel;


#[derive(Debug, Serialize, Deserialize)]
struct Pet {
    id: u64,
    name: String,
    category: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    // add logging
    std::env::set_var("RUST_LOG", "actix_web=info");
    // Initialize logger
    env_logger::init();



    // Initialize Redis connection
    let redis_url = "redis://127.0.0.1/";
    let redis_client = redis::Client::open(redis_url).expect("Failed to connect to Redis");
    let redis_connection = redis_client.get_connection().expect("Failed to get Redis connection");

    // Wrap RedisDb in a Mutex to share across threads
    let redis_db = web::Data::new(Mutex::new(db::RedisDb { client: redis_connection }));
    
    // log starting message
    log::info!("Starting server at http://localhost:8080");


    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(redis_db.clone()) // Clone the web::Data containing RedisDb
            .route("/", web::get().to(handlers::index))
            .route("/pet", web::post().to(handlers::add_pet))
            .route("/pet/{id}", web::get().to(handlers::get_pet))
            .route("/pet/{id}", web::delete().to(handlers::delete_pet))
            .route("/pet/name/{name}", web::get().to(handlers::get_pet_by_name))
            .route("/pet/category/{category}", web::get().to(handlers::get_pet_by_category))
            .route("/pet/status/{status}", web::get().to(handlers::get_pet_by_status))            
            .route("/pet/tag/{tag}", web::get().to(handlers::get_pet_by_tag))


    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
