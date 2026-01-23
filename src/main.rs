// main.rs

use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use rust_server_petstore::db::MongoDb;
use rust_server_petstore::petmodel::Pet;
use rust_server_petstore::usermodel::User;
use rust_server_petstore::config_app;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set default log level if not set
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "actix_web=info,rust_server_petstore=debug");
    }
    env_logger::init();

    // Configuration from environment variables
    let server_addr = env::var("SERVER_ADDR").or_else(|_| env::var("serverAddr")).unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    let mongo_url = env::var("DATABASE_URI").or_else(|_| env::var("databaseURI")).unwrap_or_else(|_| "mongodb://root:example@localhost:27017/?authSource=admin".to_string());

    log::info!("Starting server at {}", server_addr);
    log::info!("Connecting to MongoDB at {}", mongo_url);

    // Initialize MongoDB client
    let client = mongodb::Client::with_uri_str(&mongo_url)
        .await
        .expect("Failed to connect to MongoDB");
    let db = client.database("petstore");

    let mongo_db = MongoDb {
        pet_collection: db.collection::<Pet>("pets"),
        user_collection: db.collection::<User>("users"),
    };

    // Shared application state - web::Data is already an Arc
    let app_state = web::Data::new(mongo_db);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                let origin_str = origin.to_str().unwrap_or("");
                origin_str.ends_with(".middleland.info") || origin_str == "http://localhost:4200" || origin_str == "http://localhost:8080"
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::CONTENT_TYPE])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(cors)
            .app_data(app_state.clone())
            .configure(config_app)
    })
    .bind(&server_addr)?
    .run()
    .await
}
