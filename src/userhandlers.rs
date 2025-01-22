// userhandlers.rs

use crate::db::RedisDb;
use crate::usermodel::User;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;

// index handler to get all pets and log error message if failed
pub async fn user_index(data: web::Data<Mutex<RedisDb>>) -> impl Responder {
    // log request
    log::info!("Received request for user index");

    // get RedisDb instance from shared data
    let mut redis_db = data.lock().unwrap();
    match redis_db.get_users() {
        Ok(users) => {
            // log pets
            log::info!("Successfully retrieved users: {:?}", users);
            HttpResponse::Ok().json(users)
        }
        Err(fail) => {
            log::error!("Failed to retrieve users: {:?}", fail);
            HttpResponse::InternalServerError().finish()
        }
    }
}
// add user and log error message if failed
pub async fn add_user(
    data: web::Data<Mutex<RedisDb>>,
    new_user: web::Json<User>,
) -> impl Responder {
    // log request
    log::info!("Received request to add user {:?}", new_user);
    let mut redis_db = data.lock().unwrap();
    match redis_db.add_user(&new_user) {
        Ok(_) => {
            log::info!("Successfully added user {:?}", new_user);
            HttpResponse::Created().json(new_user.into_inner())
        }
        Err(fail) => {
            log::error!("Failed to add user {:?} , error: {:?}", new_user, fail);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// get user and log error message if failed
pub async fn get_user_by_username(
    data: web::Data<Mutex<RedisDb>>,
    username: web::Path<String>,
) -> impl Responder {
    // log request
    log::info!("Received request to get user {:?}", username);
    let mut redis_db = data.lock().unwrap();
    match redis_db.get_user(&username) {
        Ok(user) => {
            log::info!("Successfully retrieved user {:?}", user);
            HttpResponse::Ok().json(user)
        }
        Err(fail) => {
            log::error!("Failed to retrieve user {:?} , error: {:?}", username, fail);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// update user and log error message if failed
pub async fn update_user_by_username(
    data: web::Data<Mutex<RedisDb>>,
    username: web::Path<String>,
    new_user: web::Json<User>,
) -> impl Responder {
    // log request
    log::info!("Received request to update user {:?}", username);
    let mut redis_db = data.lock().unwrap();
    match redis_db.update_user(&new_user) {
        Ok(_) => {
            log::info!("Successfully updated user {:?}", new_user);
            HttpResponse::Created().json(new_user.into_inner())
        }
        Err(fail) => {
            log::error!("Failed to update user {:?} , error: {:?}", new_user, fail);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// delete user and log error message if failed
pub async fn delete_user_by_username(
    data: web::Data<Mutex<RedisDb>>,
    username: web::Path<String>,
) -> impl Responder {
    // log request
    log::info!("Received request to delete user {:?}", username);
    let mut redis_db = data.lock().unwrap();
    match redis_db.delete_user_by_username(&username) {
        Ok(_) => {
            log::info!("Successfully deleted user {:?}", username);
            HttpResponse::Ok().finish()
        }
        Err(fail) => {
            log::error!("Failed to delete user {:?} , error: {:?}", username, fail);
            HttpResponse::InternalServerError().finish()
        }
    }
}
// login user and log error message if failed
pub async fn login_user(
    data: web::Data<Mutex<RedisDb>>,
    username: web::Path<String>,
    password: web::Path<String>,
) -> impl Responder {
    // log request
    log::info!("Received request to login user {:?}", username);
    let mut redis_db = data.lock().unwrap();
    // get all users and check username and password
    match redis_db.get_users() {
        Ok(users) => {
            for user in users {
                if user.username == username.to_string() && user.password == password.to_string() {
                    log::info!("Successfully logged in user {:?}", username);
                    return HttpResponse::Ok().finish();
                }
            }
            log::info!(
                "Failed to login user {:?} , error: User not found",
                username
            );
            HttpResponse::NotFound().finish()
        }
        Err(fail) => {
            log::error!("Failed to login user {:?} , error: {:?}", username, fail);
            HttpResponse::InternalServerError().finish()
        }
    }
}
// logout user and log error message if failed
pub async fn logout_user(username: web::Path<String>) -> impl Responder {
    // log request and return Ok
    log::info!("Received request to logout user {:?}", username);
    log::info!("Successfully logged out user {:?}", username);
    HttpResponse::Ok().finish()
}
// create users with list and log error message if failed
pub async fn create_users_with_list(
    data: web::Data<Mutex<RedisDb>>,
    new_users: web::Json<Vec<User>>,
) -> impl Responder {
    // log request
    log::info!("Received request to create users {:?}", new_users);
    let mut redis_db = data.lock().unwrap();
    for user in new_users.iter() {
        match redis_db.add_user(&user) {
            Ok(_) => {
                log::info!("Successfully added user {:?}", user);
            }
            Err(fail) => {
                log::error!("Failed to add user {:?} , error: {:?}", user, fail);
                return HttpResponse::InternalServerError().finish();
            }
        }
    }
    HttpResponse::Created().finish()
}
