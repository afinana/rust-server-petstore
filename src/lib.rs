// lib.rs

pub mod db;
pub mod pethandlers;
pub mod petmodel;
pub mod userhandlers;
pub mod usermodel;

use actix_web::web;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
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
    );
}
