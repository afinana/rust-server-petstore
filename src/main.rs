// main.rs

use crate::petmodel::Pet;
use crate::usermodel::User;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use env_logger;
use std::sync::Arc;
mod db;
mod pethandlers;
mod petmodel;
mod userhandlers;
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
    let mongo_url = std::env::var("databaseURI")
        .unwrap_or("mongodb://root:example@localhost:27017/?authSource=admin".to_string());

    // Create a new MongoDB client
    let client = mongodb::Client::with_uri_str(&mongo_url)
        .await
        .expect("Failed to connect to MongoDB");
    // Get a handle to the database
    let db: mongodb::Database = client.database("petstore");

    // Get a handle to the collection
    let pets = db.collection::<Pet>("pets");
    let users = db.collection::<User>("users");

    // create mongodb instance using cliente db and collection
    let mongo_db = db::MongoDb {
        pet_collection: pets,
        user_collection: users,
    };

    // Shared application state
    let app_state = web::Data::new(Arc::new(mongo_db));

    // log starting message and server address and mongo url
    log::info!("Starting server at {} ", &server_addr);
    log::info!("Connecting to MongoDB at {}", &mongo_url);

    // Start HTTP server using the mongo_db as shared state and prefix /v2 to all routes
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://angular-petstore.middleland.info")
            .allowed_origin("https://react-petstore.middleland.info")
            .allowed_origin("https://go-gin-petstore.middleland.info")
            .allowed_origin("http://localhost:4200")
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .supports_credentials();
        App::new()
            .wrap(cors)
            .app_data(web::Data::from(app_state.clone())) // Clone the web::Data containing DB connection
            .route("/v2/pet", web::get().to(pethandlers::pet_index))
            .route("/v2/pet", web::post().to(pethandlers::add_pet))
            .route("/v2/pet", web::put().to(pethandlers::update_pet))
            .route(
                "/v2/pet/findByStatus",
                web::get().to(pethandlers::find_pet_by_status),
            )
            .route(
                "/v2/pet/findByTags",
                web::get().to(pethandlers::find_pet_by_tag),
            )
            .route("/v2/pet/{id}", web::get().to(pethandlers::get_pet))
            .route("/v2/pet/{id}", web::put().to(pethandlers::update_pet_by_id))
            .route("/v2/pet/{id}", web::delete().to(pethandlers::delete_pet))
            .route(
                "/v2/pet/name/{name}",
                web::get().to(pethandlers::get_pet_by_name),
            )
            // add user routes
            .route("/v2/user", web::get().to(userhandlers::user_index))
            .route("/v2/user", web::post().to(userhandlers::add_user))
            .route(
                "/v2/user/{username}",
                web::put().to(userhandlers::update_user_by_username),
            )
            .route("/v2/user/login", web::get().to(userhandlers::login_user))
            .route("/v2/user/logout", web::get().to(userhandlers::logout_user))
            .route(
                "/v2/user/{username}",
                web::get().to(userhandlers::get_user_by_username),
            )
            .route(
                "/v2/user/{username}",
                web::delete().to(userhandlers::delete_user_by_username),
            )
            .route(
                "/v2/user/createWithList",
                web::post().to(userhandlers::create_users_with_list),
            )
    })
    // use serverAddr from environment variable
    .bind(server_addr)?
    .run()
    .await
}
