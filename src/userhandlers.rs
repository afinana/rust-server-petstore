// userhandlers.rs

use crate::db;
use crate::usermodel::User;
use actix_web::{web, HttpResponse, Responder};

pub async fn user_index(db: web::Data<db::MongoDb>) -> impl Responder {
    log::info!("Getting all users");
    match db.get_all_users().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            log::error!("Failed to get users: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn add_user(db: web::Data<db::MongoDb>, user: web::Json<User>) -> impl Responder {
    log::info!("Adding user: {}", user.username);
    match db.add_user(&user).await {
        Ok(_) => HttpResponse::Created().json(user.into_inner()),
        Err(e) => {
            log::error!("Failed to add user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn update_user_by_username(
    db: web::Data<db::MongoDb>,
    username: web::Path<String>,
    user: web::Json<User>,
) -> impl Responder {
    log::info!("Updating user: {}", username);
    match db.update_user_by_username(&username, &user).await {
        Ok(res) if res.matched_count > 0 => HttpResponse::Ok().json(user.into_inner()),
        Ok(_) => HttpResponse::NotFound().body("User not found"),
        Err(e) => {
            log::error!("Failed to update user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_user_by_username(db: web::Data<db::MongoDb>, username: web::Path<String>) -> impl Responder {
    log::info!("Getting user: {}", username);
    match db.get_user_by_username(&username).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(e) => {
            log::error!("Failed to get user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn delete_user_by_username(db: web::Data<db::MongoDb>, username: web::Path<String>) -> impl Responder {
    log::info!("Deleting user: {}", username);
    match db.delete_user_by_username(&username).await {
        Ok(res) if res.deleted_count > 0 => HttpResponse::Ok().finish(),
        Ok(_) => HttpResponse::NotFound().body("User not found"),
        Err(e) => {
            log::error!("Failed to delete user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn create_users_with_list(db: web::Data<db::MongoDb>, users: web::Json<Vec<User>>) -> impl Responder {
    log::info!("Creating {} users", users.len());
    for user in users.iter() {
        if let Err(e) = db.add_user(user).await {
            log::error!("Failed to create user {}: {:?}", user.username, e);
            return HttpResponse::InternalServerError().finish();
        }
    }
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
pub struct LoginQuery {
    pub username: Option<String>,
    pub password: Option<String>,
}

pub async fn login_user(db: web::Data<db::MongoDb>, query: web::Query<LoginQuery>) -> impl Responder {
    let username = query.username.as_deref().unwrap_or_default();
    let password = query.password.as_deref().unwrap_or_default();

    log::info!("Login attempt for user: {}", username);
    match db.login_user(username, password).await {
        Ok(_) => HttpResponse::Ok().body("Logged in successfully"),
        Err(e) => {
            log::error!("Login failed for user {}: {:?}", username, e);
            HttpResponse::Unauthorized().body("Invalid username/password")
        }
    }
}

pub async fn logout_user(db: web::Data<db::MongoDb>, query: web::Query<LoginQuery>) -> impl Responder {
    let username = query.username.as_deref().unwrap_or_default();
    log::info!("Logout for user: {}", username);
    match db.logout_user(username).await {
        Ok(_) => HttpResponse::Ok().body("Logged out successfully"),
        Err(e) => {
            log::error!("Logout failed for user {}: {:?}", username, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}


    let result = mongo_db.logout_user(username.as_str()).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            log::error!("Failed to logout user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
