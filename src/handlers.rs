// handlers.rs

use actix_web::{web, HttpResponse, Responder};
use crate::petmodel::Pet;
use crate::db::RedisDb;
use std::sync::Mutex;

pub async fn index(data: web::Data<Mutex<RedisDb>>) -> impl Responder {
	// log request
	log::info!("Received request for index");

    if let Ok(pets) = data.lock().unwrap().get_pets() {
        HttpResponse::Ok().json(pets)
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

pub async fn add_pet(data: web::Data<Mutex<RedisDb>>, new_pet: web::Json<Pet>) -> impl Responder {
    
   // log request
   log::info!("Received request to add pet {:?}", new_pet);

   let mut redis_db = data.lock().unwrap();
    match redis_db.add_pet(&new_pet) {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_pet(data: web::Data<Mutex<RedisDb>>, path: web::Path<u64>) -> impl Responder {
   // log request
   log::info!("Received request for pet with id {}", path);

   let mut redis_db = data.lock().unwrap();
    match redis_db.get_pet_by_id(*path) {
        Ok(Some(pet)) => HttpResponse::Ok().json(pet),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_pet(data: web::Data<Mutex<RedisDb>>, path: web::Path<u64>) -> impl Responder {
	// log request
	log::info!("Received request to delete pet with id {}", path);
	
    let mut redis_db = data.lock().unwrap();
    match redis_db.delete_pet(*path) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
// add search by name
pub async fn get_pet_by_name(data: web::Data<Mutex<RedisDb>>, path: web::Path<String>) -> impl Responder {
	// log request
	log::info!("Received request for pet with name {}", path);

	let mut redis_db = data.lock().unwrap();
	match redis_db.get_pet_by_name(&path) {
		Ok(Some(pet)) => HttpResponse::Ok().json(pet),
		Ok(None) => HttpResponse::NotFound().finish(),
		Err(_) => HttpResponse::InternalServerError().finish(),
	}
}
// add search by category
pub async fn get_pet_by_category(data: web::Data<Mutex<RedisDb>>, path: web::Path<String>) -> impl Responder {
	// log request
	log::info!("Received request for pet with category {}", path);

	let mut redis_db = data.lock().unwrap();
	match redis_db.get_pet_by_category(&path) {
		Ok(pets) => HttpResponse::Ok().json(pets),
		Err(_) => HttpResponse::InternalServerError().finish(),
	}
}
// add search by status
pub async fn get_pet_by_status(data: web::Data<Mutex<RedisDb>>, path: web::Path<String>) -> impl Responder {
	// log request
	log::info!("Received request for pet with status {}", path);

	let mut redis_db = data.lock().unwrap();
	match redis_db.get_pet_by_status(&path) {
		Ok(pets) => HttpResponse::Ok().json(pets),
		Err(_) => HttpResponse::InternalServerError().finish(),
	}
}
// add search by tag
pub async fn get_pet_by_tag(data: web::Data<Mutex<RedisDb>>, path: web::Path<String>) -> impl Responder {
	// log request
	log::info!("Received request for pet with tag {}", path);

	let mut redis_db = data.lock().unwrap();
	match redis_db.get_pet_by_tag(&path) {
		Ok(pets) => HttpResponse::Ok().json(pets),
		Err(_) => HttpResponse::InternalServerError().finish(),
	}
}

