// pethandlers.rs

use crate::db;
use crate::petmodel::Pet;
use actix_web::{web, HttpResponse, Responder};

pub async fn pet_index(db: web::Data<db::MongoDb>) -> impl Responder {
    log::info!("Getting all pets");
    match db.get_all_pets().await {
        Ok(pets) => {
            log::debug!("Found {} pets", pets.len());
            HttpResponse::Ok().json(pets)
        }
        Err(e) => {
            log::error!("Failed to get pets: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn add_pet(db: web::Data<db::MongoDb>, pet: web::Json<Pet>) -> impl Responder {
    log::info!("Adding pet: {:?}", pet.name);
    match db.add_pet(&pet).await {
        Ok(_) => HttpResponse::Created().json(pet.into_inner()),
        Err(e) => {
            log::error!("Failed to add pet: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn update_pet(db: web::Data<db::MongoDb>, pet: web::Json<Pet>) -> impl Responder {
    log::info!("Updating pet: {:?}", pet.id);
    match db.update_pet(&pet).await {
        Ok(res) if res.matched_count > 0 => HttpResponse::Ok().json(pet.into_inner()),
        Ok(_) => HttpResponse::NotFound().body("Pet not found"),
        Err(e) => {
            log::error!("Failed to update pet: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn update_pet_by_id(
    db: web::Data<db::MongoDb>,
    id: web::Path<String>,
    pet: web::Json<Pet>,
) -> impl Responder {
    log::info!("Updating pet by id: {}", id);
    match db.update_pet_by_id(&id, &pet).await {
        Ok(res) if res.matched_count > 0 => HttpResponse::Ok().json(pet.into_inner()),
        Ok(_) => HttpResponse::NotFound().body("Pet not found"),
        Err(e) => {
            log::error!("Failed to update pet by id: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_pet(db: web::Data<db::MongoDb>, id: web::Path<String>) -> impl Responder {
    log::info!("Getting pet by id: {}", id);
    match db.get_pet_by_id(&id).await {
        Ok(Some(pet)) => HttpResponse::Ok().json(pet),
        Ok(None) => HttpResponse::NotFound().body("Pet not found"),
        Err(e) => {
            log::error!("Failed to get pet by id: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn delete_pet(db: web::Data<db::MongoDb>, id: web::Path<String>) -> impl Responder {
    log::info!("Deleting pet by id: {}", id);
    match db.delete_pet_by_id(&id).await {
        Ok(res) if res.deleted_count > 0 => HttpResponse::Ok().finish(),
        Ok(_) => HttpResponse::NotFound().body("Pet not found"),
        Err(e) => {
            log::error!("Failed to delete pet: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_pet_by_name(db: web::Data<db::MongoDb>, name: web::Path<String>) -> impl Responder {
    log::info!("Getting pets by name: {}", name);
    match db.get_pets_by_name(&name).await {
        Ok(pets) => HttpResponse::Ok().json(pets),
        Err(e) => {
            log::error!("Failed to get pets by name: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn find_pet_by_status(
    db: web::Data<db::MongoDb>,
    query: web::Query<StatusQuery>,
) -> impl Responder {
    log::info!("Finding pets by status: {}", query.status);
    match db.get_pets_by_status(&query.status).await {
        Ok(pets) => HttpResponse::Ok().json(pets),
        Err(e) => {
            log::error!("Failed to get pets by status: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn find_pet_by_tag(
    db: web::Data<db::MongoDb>,
    query: web::Query<TagsQuery>,
) -> impl Responder {
    log::info!("Finding pets by tags: {}", query.tags);
    match db.get_pets_by_tag(&query.tags).await {
        Ok(pets) => HttpResponse::Ok().json(pets),
        Err(e) => {
            log::error!("Failed to get pets by tags: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct StatusQuery {
    pub status: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct TagsQuery {
    pub tags: String,
}
