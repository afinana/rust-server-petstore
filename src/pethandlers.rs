// pethandlers.rs

use crate::db::RedisDb;
use crate::petmodel::Pet;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;

// index handler to get all pets and log error message if failed
pub async fn pet_index(data: web::Data<Mutex<RedisDb>>) -> impl Responder {
    // log request
    log::info!("Received request for index");

    // get RedisDb instance from shared data
    let mut redis_db = data.lock().unwrap();
    match redis_db.get_pets() {
        Ok(pets) => {
            // log pets
            log::info!("Successfully retrieved pets: {:?}", pets);
            HttpResponse::Ok().json(pets)
        }
        Err(fail) => {
            log::error!("Failed to retrieve pets: {:?}", fail);
            HttpResponse::InternalServerError().finish()
        }
    }
}
// add pet and log error message if failed
pub async fn add_pet(data: web::Data<Mutex<RedisDb>>, new_pet: web::Json<Pet>) -> impl Responder {
    // log request
    log::info!("Received request to add pet {:?}", new_pet);

    let mut redis_db = data.lock().unwrap();
    match redis_db.add_pet(&new_pet) {
        Ok(_) => {
            log::info!("Successfully added pet {:?}", new_pet);
            HttpResponse::Created().json(new_pet.into_inner())
        }
        Err(fail) => {
            log::error!("Failed to add pet {:?} , error: {:?}", new_pet, fail);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// update pet and log error message if failed
pub async fn update_pet(
    data: web::Data<Mutex<RedisDb>>,
    new_pet: web::Json<Pet>,
) -> impl Responder {
    // log request
    log::info!("Received request to update pet {:?}", new_pet);

    let mut redis_db = data.lock().unwrap();
    match redis_db.update_pet(&new_pet) {
        Ok(_) => {
            log::info!("Successfully updated pet {:?}", new_pet);
            HttpResponse::Created().json(new_pet.into_inner())
        }
        Err(fail) => {
            log::error!("Failed to update pet {:?} , error: {:?}", new_pet, fail);
            HttpResponse::InternalServerError().finish()
        }
    }
}
// update pet by id and log error message if not found
pub async fn update_pet_by_id(
    data: web::Data<Mutex<RedisDb>>,
    path: web::Path<u64>,
    new_pet: web::Json<Pet>,
) -> impl Responder {
    // log request
    log::info!(
        "Received request to update pet with id {} to {:?}",
        path,
        new_pet
    );

    let mut redis_db = data.lock().unwrap();
    match redis_db.update_pet_by_id(*path, &new_pet) {
        Ok(_) => {
            log::info!("Successfully updated pet with ID {} to {:?}", path, new_pet);
            HttpResponse::Created().json(new_pet.into_inner())
        }
        Err(fail) => {
            log::error!(
                "Failed to update pet with ID {} to {:?} , error: {:?}",
                path,
                new_pet,
                fail
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}

// get pet by id and log error message if not found
pub async fn get_pet(data: web::Data<Mutex<RedisDb>>, path: web::Path<u64>) -> impl Responder {
    // log request
    log::info!("Received request for pet with id {}", path);

    let mut redis_db = data.lock().unwrap();
    // show log error if pet not found
    match redis_db.get_pet_by_id(*path) {
        Ok(Some(pet)) => {
            log::info!("Successfully retrieved pet with ID {}", *path);
            HttpResponse::Ok().json(pet)
        }
        Ok(None) => {
            log::info!("Pet with ID {} not found", *path);
            HttpResponse::NotFound().finish()
        }
        Err(fail) => {
            log::error!(
                "Failed to retrieve pet with ID {} , error:  {:?}",
                *path,
                fail
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
// delete pet by id and log error message if not found
pub async fn delete_pet(data: web::Data<Mutex<RedisDb>>, path: web::Path<u64>) -> impl Responder {
    // log request
    log::info!("Received request to delete pet with id {}", path);

    let mut redis_db = data.lock().unwrap();
    match redis_db.delete_pet(*path) {
        Ok(_) => {
            log::info!("Successfully deleted pet with ID {}", *path);
            HttpResponse::NoContent().finish()
        }
        Err(fail) => {
            log::error!(
                "Failed to delete pet with ID {}  , error:  {:?}",
                *path,
                fail
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
// Search by name and log error message if not found
pub async fn get_pet_by_name(
    data: web::Data<Mutex<RedisDb>>,
    path: web::Path<String>,
) -> impl Responder {
    // log request
    log::info!("Received request for pet with name {}", path);

    let mut redis_db = data.lock().unwrap();
    match redis_db.get_pet_by_name(&path) {
        Ok(Some(pet)) => HttpResponse::Ok().json(pet),
        Ok(None) => {
            log::info!("Pet with name {} not found", path);
            HttpResponse::NotFound().finish()
        }
        Err(fail) => {
            log::error!("Failed to get pet by name {}  , error: {:?}", *path, fail);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Search by status query parameter and log error and success message
pub async fn find_pet_by_status(
    data: web::Data<Mutex<RedisDb>>,
    query: web::Query<StatusQuery>,
) -> impl Responder {
    // log request
    log::info!("Received request for pet with status  {:?}", query);

    let mut redis_db = data.lock().unwrap();

    match redis_db.get_pets_by_status(&query.status) {
        Ok(pets) => {
            log::info!("Successfully retrieved pets with status {:?}", query.status);
            HttpResponse::Ok().json(pets)
        }
        Err(fail) => {
            log::error!(
                "Failed to retrieve pets with status {:?} ,error: {:?}",
                query.status,
                fail
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Search by tags query parameter and log error and success message
pub async fn find_pet_by_tag(
    data: web::Data<Mutex<RedisDb>>,
    query: web::Query<TagsQuery>,
) -> impl Responder {
    // log request
    log::info!("Received request for pet with tag  {:?}", query.tags);

    let mut redis_db = data.lock().unwrap();
    match redis_db.get_pets_by_tags(&query.tags) {
        Ok(pets) => {
            log::info!("Successfully retrieved pets with tags {}", query.tags);
            HttpResponse::Ok().json(pets)
        }
        Err(fail) => {
            log::error!(
                "Failed to retrieve pets with tags {},error: {:?}",
                query.tags,
                fail
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct StatusQuery {
    status: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct TagsQuery {
    tags: String,
}
