// main.rs

mod db;
mod pethandlers;
mod petmodel;
mod userhandlers;
mod usermodel;

use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use db::MongoDb;
use petmodel::Pet;
use std::env;
use usermodel::User;

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
            .service(
                web::scope("/v2")
                    .service(
                        web::scope("/pet")
                            .route("", web::get().to(pethandlers::pet_index))
                            .route("", web::post().to(pethandlers::add_pet))
                            .route("", web::put().to(pethandlers::update_pet))
                            .route("/findByStatus", web::get().to(pethandlers::find_pet_by_status))
                            .route("/findByTags", web::get().to(pethandlers::find_pet_by_tag))
                            .route("/{id}", web::get().to(pethandlers::get_pet))
                            .route("/{id}", web::put().to(pethandlers::update_pet_by_id))
                            .route("/{id}", web::delete().to(pethandlers::delete_pet))
                            .route("/name/{name}", web::get().to(pethandlers::get_pet_by_name)),
                    )
                    .service(
                        web::scope("/user")
                            .route("", web::get().to(userhandlers::user_index))
                            .route("", web::post().to(userhandlers::add_user))
                            .route("/login", web::get().to(userhandlers::login_user))
                            .route("/logout", web::get().to(userhandlers::logout_user))
                            .route("/createWithList", web::post().to(userhandlers::create_users_with_list))
                            .route("/{username}", web::get().to(userhandlers::get_user_by_username))
                            .route("/{username}", web::put().to(userhandlers::update_user_by_username))
                            .route("/{username}", web::delete().to(userhandlers::delete_user_by_username)),
                    ),
            )
    })
    .bind(&server_addr)?
    .run()
    .await
}
