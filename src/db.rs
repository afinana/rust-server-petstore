// db.rs

use crate::petmodel::Pet;
use crate::usermodel::User;
use futures::StreamExt;
use mongodb::{
    bson::{doc, to_bson},
    error::Error,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection,
};

#[derive(Clone)]
pub struct MongoDb {
    pub pet_collection: Collection<Pet>,
    pub user_collection: Collection<User>,
}

impl MongoDb {
    pub async fn get_all_pets(&self) -> Result<Vec<Pet>, Error> {
        let mut cursor = self.pet_collection.find(doc! {}).await?;
        let mut pets = vec![];
        while let Some(result) = cursor.next().await {
            pets.push(result?);
        }
        Ok(pets)
    }

    pub async fn get_pet_by_id(&self, id: &str) -> Result<Option<Pet>, Error> {
        let id_val = id.parse::<i64>().map_err(|e| {
            Error::from(std::io::Error::new(std::io::ErrorKind::InvalidInput, e))
        })?;
        let filter = doc! { "id": id_val };
        self.pet_collection.find_one(filter).await
    }

    pub async fn get_pets_by_name(&self, name: &str) -> Result<Vec<Pet>, Error> {
        let filter = doc! { "name": name };
        let mut cursor = self.pet_collection.find(filter).await?;
        let mut pets = vec![];
        while let Some(result) = cursor.next().await {
            pets.push(result?);
        }
        Ok(pets)
    }

    pub async fn add_pet(&self, pet: &Pet) -> Result<InsertOneResult, Error> {
        self.pet_collection.insert_one(pet).await
    }

    pub async fn update_pet(&self, pet: &Pet) -> Result<UpdateResult, Error> {
        let filter = doc! { "id": pet.id as i64 };
        let pet_bson = to_bson(pet)?;
        let update = doc! { "$set": pet_bson };
        self.pet_collection.update_one(filter, update).await
    }

    pub async fn get_pets_by_tag(&self, tag_str: &str) -> Result<Vec<Pet>, Error> {
        let tags: Vec<&str> = tag_str.split(',').collect();
        let filter = doc! { "tags.name": { "$in": tags } };
        let mut cursor = self.pet_collection.find(filter).await?;
        let mut pets = vec![];
        while let Some(result) = cursor.next().await {
            pets.push(result?);
        }
        Ok(pets)
    }

    pub async fn get_pets_by_status(&self, status_str: &str) -> Result<Vec<Pet>, Error> {
        let statuses: Vec<&str> = status_str.split(',').collect();
        let filter = doc! { "status": { "$in": statuses } };
        let mut cursor = self.pet_collection.find(filter).await?;
        let mut pets = vec![];
        while let Some(result) = cursor.next().await {
            pets.push(result?);
        }
        Ok(pets)
    }

    pub async fn delete_pet_by_id(&self, id: &str) -> Result<DeleteResult, Error> {
        let id_val = id.parse::<i64>().map_err(|e| {
            Error::from(std::io::Error::new(std::io::ErrorKind::InvalidInput, e))
        })?;
        let filter = doc! { "id": id_val };
        self.pet_collection.delete_one(filter).await
    }

    pub async fn update_pet_by_id(&self, id: &str, pet: &Pet) -> Result<UpdateResult, Error> {
        let id_val = id.parse::<i64>().map_err(|e| {
            Error::from(std::io::Error::new(std::io::ErrorKind::InvalidInput, e))
        })?;
        let filter = doc! { "id": id_val };
        let pet_bson = to_bson(pet)?;
        let update = doc! { "$set": pet_bson };
        self.pet_collection.update_one(filter, update).await
    }

    pub async fn add_user(&self, user: &User) -> Result<InsertOneResult, Error> {
        self.user_collection.insert_one(user).await
    }

    pub async fn delete_user_by_username(&self, username: &str) -> Result<DeleteResult, Error> {
        let filter = doc! { "username": username };
        self.user_collection.delete_one(filter).await
    }

    pub async fn update_user_by_username(&self, username: &str, user: &User) -> Result<UpdateResult, Error> {
        let filter = doc! { "username": username };
        let user_bson = to_bson(user)?;
        let update = doc! { "$set": user_bson };
        self.user_collection.update_one(filter, update).await
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut cursor = self.user_collection.find(doc! {}).await?;
        let mut users = vec![];
        while let Some(result) = cursor.next().await {
            users.push(result?);
        }
        Ok(users)
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, Error> {
        let filter = doc! { "username": username };
        self.user_collection.find_one(filter).await
    }

    pub async fn login_user(&self, username: &str, password: &str) -> Result<UpdateResult, Error> {
        let filter = doc! { "username": username, "password": password };
        let user = self.user_collection.find_one(filter).await?;
        match user {
            Some(_) => {
                let filter = doc! { "username": username };
                let update = doc! { "$set": { "logged_in": true } };
                self.user_collection.update_one(filter, update).await
            }
            None => Err(Error::from(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "User not found or password incorrect",
            ))),
        }
    }

    pub async fn logout_user(&self, username: &str) -> Result<UpdateResult, Error> {
        let filter = doc! { "username": username };
        let update = doc! { "$set": { "logged_in": false } };
        self.user_collection.update_one(filter, update).await
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_id_parsing() {
        let id = "123";
        let parsed = id.parse::<i64>();
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap(), 123);

        let invalid_id = "abc";
        let parsed_invalid = invalid_id.parse::<i64>();
        assert!(parsed_invalid.is_err());
    }
}

