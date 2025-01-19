// main.rs

use crate::petmodel::Pet;
use crate::usermodel::User;

use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use env_logger;

mod db;
mod pethandlers;
mod userhandlers;
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
    let redis_url = std::env::var("redisURI").unwrap_or("redis://localhost/".to_string());
    
    let redis_client = redis::Client::open(redis_url).expect("Failed to connect to Redis");
    let redis_connection = redis_client.get_connection().expect("Failed to get Redis connection");

    // Wrap RedisDb in a Mutex to share across threads
    let redis_db = web::Data::new(Mutex::new(db::RedisDb { client: redis_connection }));
    
    // log starting message and server address and redis url
    log::info!("Starting server at {} ",&server_addr);
    

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(redis_db.clone()) // Clone the web::Data containing RedisDb
            .route("/v2/pet", web::get().to(pethandlers::pet_index))
            .route("/v2/pet", web::post().to(pethandlers::add_pet))
            .route("/v2/pet", web::put().to(pethandlers::update_pet))
            .route("/v2/pet/findByStatus", web::get().to(pethandlers::find_pet_by_status))            
            .route("/v2/pet/findByTags", web::get().to(pethandlers::find_pet_by_tag))            
            .route("/v2/pet/{id}", web::get().to(pethandlers::get_pet))
            .route("/v2/pet/{id}", web::put().to(pethandlers::update_pet_by_id))         
            .route("/v2/pet/{id}", web::delete().to(pethandlers::delete_pet))
            .route("/v2/pet/name/{name}", web::get().to(pethandlers::get_pet_by_name))
            // add user routes
            .route("/v2/user", web::get().to(userhandlers::user_index))
            .route("/v2/user", web::post().to(userhandlers::add_user))
            .route("/v2/user/{username}", web::put().to(userhandlers::update_user_by_username))
            .route("/v2/user/login", web::get().to(userhandlers::login_user))
            .route("/v2/user/logout", web::get().to(userhandlers::logout_user))            
            .route("/v2/user/{username}", web::get().to(userhandlers::get_user_by_username))
            .route("/v2/user/{username}", web::delete().to(userhandlers::delete_user_by_username))
            .route("/v2/user/createWithList", web::post().to(userhandlers::create_users_with_list))
            

    })
    // use serverAddr from environment variable
    .bind(server_addr)?
    .run()
    .await
}
