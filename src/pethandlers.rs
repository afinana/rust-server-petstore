// pethandlers.rs

use crate::db;
use crate::petmodel::Pet;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

// index handler to get all pets and log error message if failed using crate::db::MongoDb
pub async fn pet_index(mongo_db: web::Data<Arc<db::MongoDb>>) -> impl Responder {
    // add log start message
    log::info!("Getting all pets");

    let pets = mongo_db.get_all_pets().await;
    match pets {
        Ok(pets) => {
            log::info!("Found pets: {:?}", pets.len());
            HttpResponse::Ok().json(pets)
        }
        Err(e) => {
            log::error!("Failed to get pets: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// add pet and log error message if failed
pub async fn add_pet(mongo_db: web::Data<Arc<db::MongoDb>>, pet: web::Json<Pet>) -> impl Responder {
    // add log start message
    log::info!("Adding pet: {:?}", pet);

    let result = mongo_db.add_pet(&pet).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            log::error!("Failed to add pet: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
// update a pet and log error message if failed
pub async fn update_pet(
    mongo_db: web::Data<Arc<db::MongoDb>>,
    pet: web::Json<Pet>,
) -> impl Responder {
    // add log start message
    log::info!("Updating pet: {:?}", pet);

    let result = mongo_db.update_pet(&pet).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            log::error!("Failed to update pet: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
// update a pet by id and log error message if failed
pub async fn update_pet_by_id(
    mongo_db: web::Data<Arc<db::MongoDb>>,
    id: web::Path<String>,
    pet: web::Json<Pet>,
) -> impl Responder {
    // add log start message
    log::info!("Updating pet by id: {:?}", id);

    let result = mongo_db.update_pet_by_id(id.as_str(), &pet).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            log::error!("Failed to update pet by id: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// get pet by id and log error message if not found
pub async fn get_pet(
    mongo_db: web::Data<Arc<db::MongoDb>>,
    id: web::Path<String>,
) -> impl Responder {
    // add log start message
    log::info!("Getting pet by id: {:?}", id);

    let pet = mongo_db.get_pet_by_id(&id).await;
    match pet {
        Some(pet) => {
            log::info!("Found pet by id: {:?}", id);
            HttpResponse::Ok().json(pet)
        }
        None => {
            log::error!("Pet not found by id: {:?}", id);
            HttpResponse::NotFound().finish()
        }
    }
}

// delete pet by id and log error message if not found
pub async fn delete_pet(
    mongo_db: web::Data<Arc<db::MongoDb>>,
    id: web::Path<String>,
) -> impl Responder {
    // add log start message
    log::info!("Deleting pet by id: {:?}", id);

    let result = mongo_db.delete_pet_by_id(id.as_str()).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            log::error!("Failed to delete pet: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Search by name and log error message if not found
pub async fn get_pet_by_name(
    mongo_db: web::Data<Arc<db::MongoDb>>,
    query: web::Query<NameQuery>,
) -> impl Responder {
    // add log start message
    log::info!("Getting pet by name: {:?}", query.name);

    let pets = mongo_db.get_pets_by_name(&query.name).await;
    match pets {
        Ok(pets) => {
            log::info!("Found pets by name: {:?}", query.name);
            HttpResponse::Ok().json(pets)
        }
        Err(e) => {
            log::error!("Failed to get pets by name: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Search by status query parameter and log error and success message
pub async fn find_pet_by_status(
    mongo_db: web::Data<Arc<db::MongoDb>>,
    query: web::Query<StatusQuery>,
) -> impl Responder {
    // add log start message
    log::info!("Finding pet by status: {:?}", query.status);

    let pets = mongo_db.get_pets_by_status(&query.status).await;
    match pets {
        Ok(pets) => {
            // add log success message with number of pets found
            log::info!("Found pets by status: {:?}", pets.len());
            HttpResponse::Ok().json(pets)
        }
        Err(e) => {
            log::error!("Failed to get pets by status: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Search by tags query parameter and log error and success message
pub async fn find_pet_by_tag(
    mongo_db: web::Data<Arc<db::MongoDb>>,
    query: web::Query<TagsQuery>,
) -> impl Responder {
    // add log start message
    log::info!("Finding pet by tags: {:?}", query.tags);

    let pets = mongo_db.get_pets_by_tag(&query.tags).await;
    match pets {
        Ok(pets) => {
            // add log success message with number of pets found
            log::info!("Found pets by tags: {:?}", pets.len());
            HttpResponse::Ok().json(pets)
        }
        Err(e) => {
            log::error!("Failed to get pets by tags: {:?}", e);
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

#[derive(Debug, serde::Deserialize)]
pub struct NameQuery {
    name: String,
}
