// pethandlers.rs

use crate::db;
use std::sync::Mutex;
use actix_web::{web, HttpResponse, Responder};
use crate::petmodel::Pet;


// index handler to get all pets and log error message if failed
pub async fn pet_index(data: web::Data<Mutex<RedisDb>>) -> impl Responder {
	// log request
	log::info!("Received request for index");

	// get RedisDb instance from shared data
	let mut redis_db = data.lock().unwrap();
	match redis_db.get_pets() {
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
pub async fn add_pet(data: web::Data<Mutex<RedisDb>>, new_pet: web::Json<Pet>) -> impl Responder {
    
   // log request
   log::info!("Received request to add pet {:?}", new_pet);

   let mut redis_db = data.lock().unwrap();
   match redis_db.add_pet(&new_pet) {
        Ok(_) => {
            log::info!("Successfully added pet {:?}", new_pet);
            HttpResponse::Created().json(new_pet.into_inner())
        },
        Err(fail) => {
            log::error!("Failed to add pet {:?} , error: {:?}", new_pet, fail);
            HttpResponse::InternalServerError().finish()
        },
    }
}

// update pet and log error message if failed
pub async fn update_pet(data: web::Data<Mutex<RedisDb>>, new_pet: web::Json<Pet>) -> impl Responder {
	// log request
	log::info!("Received request to update pet {:?}", new_pet);

	let mut redis_db = data.lock().unwrap();
	match redis_db.update_pet(&new_pet) {
		Ok(_) => {
			log::info!("Successfully updated pet {:?}", new_pet);
			HttpResponse::Created().json(new_pet.into_inner())
		},
		Err(fail) => {
			log::error!("Failed to update pet {:?} , error: {:?}", new_pet, fail);
			HttpResponse::InternalServerError().finish()
		},
	}
}
// update pet by id and log error message if not found
pub async fn update_pet_by_id(data: web::Data<Mutex<RedisDb>>, path: web::Path<u64>, new_pet: web::Json<Pet>) -> impl Responder {
	// log request
	log::info!("Received request to update pet with id {} to {:?}", path, new_pet);

	let mut redis_db = data.lock().unwrap();
	match redis_db.update_pet_by_id(*path, &new_pet) {
		Ok(_) => {
			log::info!("Successfully updated pet with ID {} to {:?}", path, new_pet);
			HttpResponse::Created().json(new_pet.into_inner())
		},
		Err(fail) => {
			log::error!("Failed to update pet with ID {} to {:?} , error: {:?}", path, new_pet, fail);
			HttpResponse::InternalServerError().finish()
		},
	}
}

// get pet by id and log error message if not found
pub async fn get_pet(mongo_db: web::Data<Mutex<db::MongoDb>>, id: web::Path<String>) -> impl Responder {
	// add log start message
	log::info!("Getting pet by id: {:?}", id);

	let mongo_db = mongo_db.lock().unwrap();
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
pub async fn delete_pet(mongo_db: web::Data<Mutex<db::MongoDb>>, id: web::Path<String>) -> impl Responder {
	// add log start message
	log::info!("Deleting pet by id: {:?}", id);
	
	let mongo_db = mongo_db.lock().unwrap();
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
pub async fn get_pet_by_name(mongo_db: web::Data<Mutex<db::MongoDb>>, query: web::Query<NameQuery>) -> impl Responder {
	// add log start message
	log::info!("Getting pet by name: {:?}", query.name);

	let mongo_db = mongo_db.lock().unwrap();
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
pub async fn find_pet_by_status(mongo_db: web::Data<Mutex<db::MongoDb>>, query: web::Query<StatusQuery>) -> impl Responder {
	// add log start message
	log::info!("Finding pet by status: {:?}", query.status);

	let mongo_db = mongo_db.lock().unwrap();
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
pub async fn find_pet_by_tag(data: web::Data<Mutex<RedisDb>>, query: web::Query<TagsQuery>) -> impl Responder {
	// log request
	log::info!("Received request for pet with tag  {:?}", query.tags);

	let mut redis_db = data.lock().unwrap();
	match redis_db.get_pets_by_tags(&query.tags) {
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