use crate::db;
use std::sync::Arc;
use actix_web::{web, HttpResponse, Responder};
use crate::usermodel::User;


// index handler to get all users and log error message if failed using crate::db::MongoDb
pub async fn user_index(mongo_db: web::Data<Arc<db::MongoDb>>) -> impl Responder {
	// add log start message
	log::info!("Getting all users");

	let users = mongo_db.get_all_users().await;
	match users {
		Ok(users) => {
			log::info!("Found users: {:?}", users.len());
			HttpResponse::Ok().json(users)
		}
		Err(e) => {
			log::error!("Failed to get users: {:?}", e);
			HttpResponse::InternalServerError().finish()
		}
	}
}

// add user and log error message if failed
pub async fn add_user(mongo_db: web::Data<Arc<db::MongoDb>>, user: web::Json<User>) -> impl Responder {
	// add log start message
	log::info!("Adding user: {:?}", user);

	let result = mongo_db.add_user(&user).await;
	match result {
		Ok(_) => HttpResponse::Ok().finish(),
		Err(e) => {
			log::error!("Failed to add user: {:?}", e);
			HttpResponse::InternalServerError().finish()
		}
	}
}

// update a user by username and log error message if failed
pub async fn update_user_by_username(mongo_db: web::Data<Arc<db::MongoDb>>,username: web::Path<String>) -> impl Responder {
	
	// add log start message
	log::info!("Updating user by username: {:?}",username);
	
	let result = mongo_db.update_user_by_username(username.as_str()).await;
	match result {
		Ok(_) => HttpResponse::Ok().finish(),
		Err(e) => {
			log::error!("Failed to update user by username: {:?}", e);
			HttpResponse::InternalServerError().finish()
		}
	}
}
// get user by username and log error message if failed
pub async fn get_user_by_username(mongo_db: web::Data<Arc<db::MongoDb>>, username: web::Path<String>) -> impl Responder {
	// add log start message
	log::info!("Getting user by username: {:?}", username);
	
	let user = mongo_db.get_user_by_username(username.as_str()).await;
	match user {
		Some(user) => HttpResponse::Ok().json(user),
		None => HttpResponse::NotFound().finish(),
	}
}
// delete user by username and log error message if failed
pub async fn delete_user_by_username(mongo_db: web::Data<Arc<db::MongoDb>>, username: web::Path<String>) -> impl Responder {
	// add log start message
	log::info!("Deleting user by username: {:?}", username);
	
	let result = mongo_db.delete_user_by_username(username.as_str()).await;
	match result {
		Ok(_) => HttpResponse::Ok().finish(),
		Err(e) => {
			log::error!("Failed to delete user by username: {:?}", e);
			HttpResponse::InternalServerError().finish()
		}
	}
}
// create users with list input and log error message if failed
pub async fn create_users_with_list(mongo_db: web::Data<Arc<db::MongoDb>>, users: web::Json<Vec<User>>) -> impl Responder {
	// add log start message
	log::info!("Creating users with list input: {:?}", users);
	
	// create user for each user in the list
	for user in users.iter() {
		let result = mongo_db.add_user(user).await;
		match result {
			Ok(_) => (),
			Err(e) => {
				log::error!("Failed to create user with list input: {:?}", e);
				return HttpResponse::InternalServerError().finish();
			}
		}
	}
	HttpResponse::Ok().finish()
	
}
// login user and log error message if failed
pub async fn login_user(mongo_db: web::Data<Arc<db::MongoDb>>, username: web::Path<String>, password: web::Path<String>) -> impl Responder {
	// add log start message
	log::info!("Logging in user: {:?}", username);
	
	let result = mongo_db.login_user(username.as_str(), password.as_str()).await;
	match result {
		Ok(_) => HttpResponse::Ok().finish(),
		Err(e) => {
			log::error!("Failed to login user: {:?}", e);
			HttpResponse::InternalServerError().finish()
		}
	}
}
// logout user and log error message if failed
pub async fn logout_user(mongo_db: web::Data<Arc<db::MongoDb>>, username: web::Path<String>) -> impl Responder {
	// add log start message
	log::info!("Logging out user: {:?}", username);
	
	let result = mongo_db.logout_user(username.as_str()).await;
	match result {
		Ok(_) => HttpResponse::Ok().finish(),
		Err(e) => {
			log::error!("Failed to logout user: {:?}", e);
			HttpResponse::InternalServerError().finish()
		}
	}
}


