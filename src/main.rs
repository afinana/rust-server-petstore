// main.rs

use crate::petmodel::Pet;
use crate::usermodel::User;

use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use env_logger;

mod db;
mod handlers;
mod petmodel;
mod usermodel;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    // add logging
    std::env::set_var("RUST_LOG", "actix_web=info");
    // Initialize logger
    env_logger::Builder::new()
       .filter_level(log::LevelFilter::Debug)
      .init();
   
    // Create serverAddr from environment variable
    let server_addr = std::env::var("serverAddr").unwrap_or("localhost:8080".to_string());  
  
    // Initialize Redis connection using a environment variable
    let mongo_url = std::env::var("mongoURI").unwrap_or("mongodb://root:example@localhost:27017/?authSource=admin".to_string());
    
    // Create a new MongoDB client
    let client = mongodb::Client::with_uri_str(&mongo_url).await.expect("Failed to connect to MongoDB");
    // Get a handle to the database
    let db: mongodb::Database = client.database("petstore");
    
    // Get a handle to the collection
    let pets = db.collection::<Pet>("pets");
    let users = db.collection::<User>("users");

    // create mongodb instance using cliente db and collection
    let mongo_db = db::MongoDb {
        client,
        db,
        pet_collection: pets,
        user_collection: users,
    };

    
    // Create a mutex to share the RedisDb across multiple requests
    let mongo_db = web::Data::new(Mutex::new(mongo_db));

     // log starting message and server address and mongo url
     log::info!("Starting server at {} ",&server_addr);
     log::info!("Connecting to MongoDB at {}", &mongo_url);
    
    // Start HTTP server using the mongo_db as shared state

    HttpServer::new(move || {
        App::new()
            .app_data(mongo_db.clone()) // Clone the web::Data containing RedisDb
            .route("/pet", web::get().to(handlers::index))
            .route("/pet", web::post().to(handlers::add_pet))
            .route("/pet", web::put().to(handlers::update_pet))
            .route("/pet/findByStatus", web::get().to(handlers::find_pet_by_status))            
            .route("/pet/findByTags", web::get().to(handlers::get_pet_by_tag))            
            .route("/pet/{id}", web::get().to(handlers::get_pet))
            .route("/pet/{id}", web::put().to(handlers::update_pet_by_id))         
            .route("/pet/{id}", web::delete().to(handlers::delete_pet))
            .route("/pet/name/{name}", web::get().to(handlers::get_pet_by_name))          



    })
    // use serverAddr from environment variable
    .bind(server_addr)?
    .run()
    .await
}
