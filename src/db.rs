// db.rs
// change all methods using mongodb instead of redis

use crate::petmodel::Pet;
use crate::usermodel::User;

use futures::StreamExt;
use mongodb::{
    bson::{doc, to_bson }, 
	error::Error, 
	results::{DeleteResult, InsertOneResult, UpdateResult}, 
	Collection,
	Database

};

pub struct MongoDb {
	pub client: mongodb::Client,
	pub db: Database,
	pub pet_collection: Collection<Pet>,
	pub user_collection: Collection<User>,
}

impl MongoDb {

	// get all pets from the collection
	pub async fn get_all_pets(&self) -> Result<Vec<Pet>, Error> {
		let mut cursor = self.pet_collection.find(None, None).await?;
		let mut pets: Vec<Pet> = vec![];
		while let Some(result) = cursor.next().await {
			match result {
				Ok(document) => {					
					pets.push(document);
				}
				Err(e) => return Err(e),
			}
		}
		Ok(pets)
	}
	// let filter = doc! { "_id": id.into() };
	// get pet by id from the collection
	pub async fn get_pet_by_id(&self, id: &str) -> Option<Pet> {

		// create a filter convert id to i64
		let filter = doc! { "id": id.parse::<i64>().unwrap() };

		// find one pet by id and convert to pet struct
		let pet = self.pet_collection.find_one(filter, None).await.unwrap();
		match pet {
			Some(pet) => Some(pet),
			None => None,
		}
		
	}
	// get all pets by name
	pub async fn get_pets_by_name(&self, name: &str) -> Result<Vec<Pet>, Error> {
		let filter = doc! { "name": name };
		let mut cursor = self.pet_collection.find(filter, None).await?;
		let mut pets: Vec<Pet> = vec![];
		while let Some(result) = cursor.next().await {
			match result {
				Ok(document) => {					
					pets.push(document);
				}
				Err(e) => return Err(e),
			}
		}
		Ok(pets)
	}
	
	// add pet to the collection
	pub async fn add_pet(&self, pet: &Pet) -> Result<InsertOneResult, Error> {		
		self.pet_collection.insert_one(pet, None).await
	}
	// update pet in the collection
	pub async fn update_pet(&self, pet: &Pet) -> Result<UpdateResult, Error> {
		let filter = doc! { "id": pet.id as i64};
		let pet_bson = to_bson(pet).unwrap(); // Replace `unwrap` with proper error handling.
 
		let update = doc! { "$set": pet_bson};			
		self.pet_collection.update_one(filter, update, None).await
	}


	// search pet by tag from the collection
	pub async fn get_pets_by_tag(&self, tag: &str) -> Result<Vec<Pet>, Error> {

	   // split tag by comma
	   let tags: Vec<&str> = tag.split(",").collect();
	   // for each tag create a filter with elemMatch
	   let mut filters = vec![];
	   for tag in tags {
		   let filter = doc! { "tags": { "$elemMatch": { "name": tag } } };
		   filters.push(filter);
	   }
	   // create a filter with or operator
	   let filter = doc! { "$or": filters };
	   let mut cursor = self.pet_collection.find(filter, None).await?;
	   let mut pets: Vec<Pet> = vec![];
	   while let Some(result) = cursor.next().await {
			   match result {
				   Ok(document) => {					
					   pets.push(document);
				   }
				   Err(e) => return Err(e),
			   }
	   }
	   Ok(pets)
	}
	

	// search pet by status from the collection
	pub async fn get_pets_by_status(&self, status: &str) -> Result<Vec<Pet>, Error> {
		
	   // split tag by comma
	   let status_v: Vec<&str> = status.split(",").collect();
	   // for each tag create a filter with elemMatch
	   let mut filters = vec![];
	   for status in status_v {
		   let filter = doc! { "status": status };
		   filters.push(filter);
	   }
	   // create a filter with or operator
	   let filter = doc! { "$or": filters };
	   let mut cursor = self.pet_collection.find(filter, None).await?;
	   let mut pets: Vec<Pet> = vec![];
	   while let Some(result) = cursor.next().await {
			   match result {
				   Ok(document) => {					
					   pets.push(document);
				   }
				   Err(e) => return Err(e),
			   }
	   }
	   Ok(pets)
	}
	// delete a pey by id from the collection
	pub async fn delete_pet_by_id(&self, id: &str) -> Result<DeleteResult, Error> {
		// create a filter convert id to i64
		let filter = doc! { "id": id.parse::<i64>().unwrap() };
		self.pet_collection.delete_one(filter, None).await
	}
	
	
	// update a pet by id from the collection 
	pub async fn update_pet_by_id(&self, id: &str, pet: &Pet) -> Result<UpdateResult, Error> {
		// create a filter convert id to i64
		let filter = doc! { "id": id.parse::<i64>().unwrap() };
		let pet_bson = to_bson(pet).unwrap(); // Replace `unwrap` with proper error handling.
 
		let update = doc! { "$set": pet_bson};			
		self.pet_collection.update_one(filter, update, None).await

	}


	// add user to the collection
	pub async fn add_user(&self, user: &User) -> Result<InsertOneResult, Error> {
		self.user_collection.insert_one(user, None).await
	}
	// update user in the collection
	//pub async fn update_user(&self, user: &User) -> Result<UpdateResult, Error> {
	//	let filter = doc! { "username": user.username.as_str() };
	//	let user_bson = to_bson(user).unwrap(); // Replace `unwrap` with proper error handling.
	//	let update = doc! { "$set": user_bson};			
	//	self.user_collection.update_one(filter, update, None).await
	//}

	// delete user by username from the collection
	pub async fn delete_user_by_username(&self, username: &str) -> Result<DeleteResult, Error> {
		let filter = doc! { "username": username };
		self.user_collection.delete_one(filter, None).await
	}
	// update user by username from the collection
	pub async fn update_user_by_username(&self, username: &str) -> Result<UpdateResult, Error> {
		let filter = doc! { "username": username };
		// find one user by username and convert to user struct
		let user = self.user_collection.find_one(filter.clone(), None).await.unwrap();
		match user {
			Some(user) => {
				let user_bson = to_bson(&user).unwrap(); // Replace `unwrap` with proper error handling.
				let update = doc! { "$set": user_bson};	
			
				self.user_collection.update_one(filter.clone(), update, None).await
			}
			None => Err(Error::from(std::io::Error::new(std::io::ErrorKind::NotFound, "User not found"))),
		}
	}
		
	// get all users from the collection
	pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
		let mut cursor = self.user_collection.find(None, None).await?;
		let mut users: Vec<User> = vec![];
		while let Some(result) = cursor.next().await {
			match result {
				Ok(document) => {					
					users.push(document);
				}
				Err(e) => return Err(e),
			}
		}
		Ok(users)
	}
	// get user by username from the collection
	pub async fn get_user_by_username(&self, username: &str) -> Option<User> {
		let filter = doc! { "username": username };
		let user = self.user_collection.find_one(filter, None).await.unwrap();
		match user {
			Some(user) => Some(user),
			None => None,
		}
	}
	// login user from the collection
	pub async fn login_user(&self, username: &str, password: &str) -> Result<UpdateResult, Error> {
		let filter = doc! { "username": username, "password": password };
		let _user = self.user_collection.find_one(filter, None).await.unwrap();
		match _user {
			Some(_user) => {
				let filter = doc! { "username": username };
				let update = doc! { "$set": { "logged_in": true } };
				self.user_collection.update_one(filter, update, None).await
			}
			None => Err(Error::from(std::io::Error::new(std::io::ErrorKind::NotFound, "User not found"))),
		}
		
	}
	// logout user from the collection
	pub async fn logout_user(&self, username: &str) -> Result<UpdateResult, Error> {
		let filter = doc! { "username": username };
		let update = doc! { "$set": { "logged_in": false } };
		self.user_collection.update_one(filter, update, None).await
	}
		
}

