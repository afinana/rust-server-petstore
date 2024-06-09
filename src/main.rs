// main.rs

use actix_web::{web, App, HttpServer};

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use env_logger;

use crate::petmodel::AppData;
mod db;
mod handlers;
mod petmodel;
mod usermodel;
mod mq;



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
    let redis_url = std::env::var("redisURI").unwrap_or("redis://127.0.0.1/".to_string());
    
    let redis_client = redis::Client::open(redis_url).expect("Failed to connect to Redis");
    let redis_connection = redis_client.get_connection().expect("Failed to get Redis connection");

    // Wrap RedisDb in a Mutex to share across threads
    let redis = web::Data::new(Mutex::new(db::RedisDb { client: redis_connection }));

    // Wrap RabbitMQ in a Mutex to share across threads
    let rabbitMQ: web::Data<Mutex<RabbitMQ>> = web::Data::new(Mutex::new(mq::RabbitMQ {}));



    // Create a Rabbit MQ connection using a environment variable
    //let rabbitmq_url = std::env::var("rabbitMQURI").unwrap_or("amqp://localhost:5672".to_string());
    
   // Create a Rabbit mq object
    
	
   
   // create an object with the redis_db and mq
   
   let app_data = web::Data::new(AppData {
        redis_db: redis,
		mq: rabbitMQ,
	});

    
    // log starting message and server address and redis url
    log::info!("Starting server at {} ",&server_addr);
    

    // Start HTTP server    
    HttpServer::new(move || {
        App::new()
			.app_data(app_data.clone()) // Clone the web::Data containing AppData            
            .route("/v2/pet", web::get().to(handlers::index))
            .route("/v2/pet", web::post().to(handlers::add_pet))
            .route("/v2/pet", web::put().to(handlers::update_pet))
            .route("/v2/pet/findByStatus", web::get().to(handlers::find_pet_by_status))            
            .route("/v2/pet/findByTags", web::get().to(handlers::get_pet_by_tag))            
            .route("/v2/pet/{id}", web::get().to(handlers::get_pet))
            .route("/v2/pet/{id}", web::put().to(handlers::update_pet_by_id))         
            .route("/v2/pet/{id}", web::delete().to(handlers::delete_pet))
            .route("/v2/pet/name/{name}", web::get().to(handlers::get_pet_by_name))


    })
    // use serverAddr from environment variable
    .bind(server_addr)?
    .run()
    .await
    
}
