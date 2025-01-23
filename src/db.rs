// db.rs

use crate::petmodel::Pet;
use crate::usermodel::User;
use log::error;
use redis::Commands;
use serde_json::Error as SerdeError;

pub struct RedisDb {
    pub client: redis::Client,
}

impl RedisDb {
    fn serialize<T: serde::Serialize>(value: &T) -> Result<String, SerdeError> {
        serde_json::to_string(value)
    }

    fn deserialize<'a, T: serde::Deserialize<'a>>(s: &'a str) -> Result<T, SerdeError> {
        serde_json::from_str(s)
    }

    pub fn add_pet(&mut self, pet: &Pet) -> redis::RedisResult<()> {


        // get connection from pool of 
        let mut conn = self.client.get_connection().expect("Failed to get connection");

        let pet_json = Self::serialize(pet).map_err(|e| {
            error!("Serialization error: {}", e);
            redis::RedisError::from((redis::ErrorKind::TypeError, "Serialization error"))
        })?;
        log::info!("Received request to add pet {:?}", pet_json);

        let _: () = conn.hset("pets", pet.id, pet_json)?;
        log::info!("Added to hset pets");

        let _: () = conn.hset("pet_names", pet.name.clone(), pet.id)?;
        log::info!("Added to hset names");

        // add pet status to set
        let status_key = format!("pet_status:{}", pet.status.clone());
        log::info!("Status key: {}", status_key);

        let _: () = conn.sadd(status_key, pet.id)?;
        log::info!("Added to set pet_status");

        if let Some(tags) = &pet.tags {
            for tag in tags {
                let tag_key = format!("pet_tag:{}", tag.name.clone());
                // log the tag key
                log::info!("Tag key: {}", tag_key);
                let _: () = conn.sadd(tag_key, pet.id)?;
                log::info!("Added to tag set");
            }
        }

        Ok(())
    }

    pub fn update_pet(&mut self, pet: &Pet) -> redis::RedisResult<()> {
        if let Some(my_pet) = self.get_pet_by_id(pet.id)? {
            self.delete_pet(my_pet.id)?;
        }
        self.add_pet(pet)
    }

    pub fn update_pet_by_id(&mut self, id: u64, pet: &Pet) -> redis::RedisResult<()> {
        log::info!("update_pet_by_id: {}", id);
        self.add_pet(pet)
    }

    pub fn get_pets(&mut self) -> redis::RedisResult<Vec<Pet>> {
        // get connection from pool of 
        let mut conn = self.client.get_connection().expect("Failed to get connection");

        let pet_map: std::collections::HashMap<String, String> = conn.hgetall("pets")?;
        let pets: Vec<Pet> = pet_map
            .values()
            .filter_map(|json| Self::deserialize(json).ok())
            .collect();
        Ok(pets)
    }

    pub fn get_pet_by_id(&mut self, id: u64) -> redis::RedisResult<Option<Pet>> {
        
        // get connection from pool of 
        let mut conn = self.client.get_connection().expect("Failed to get connection");

        let pet_json: Option<String> =conn.hget("pets", id)?;
        pet_json.map_or(Ok(None), |json| {
            Self::deserialize(&json).map(Some).map_err(|e| {
                error!("Deserialization error: {}", e);
                redis::RedisError::from((redis::ErrorKind::TypeError, "Deserialization error"))
            })
        })
    }

    pub fn delete_pet(&mut self, id: u64) -> redis::RedisResult<()> {
        
        // get connection from pool of 
        let mut conn = self.client.get_connection().expect("Failed to get connection");

        if let Some(pet) = self.get_pet_by_id(id)? {
            let _: () = conn.hdel("pet_names", pet.name)?;
            let _: () = conn.srem(format!("pet_status:{}", pet.status.clone()), id)?;
            if let Some(tags) = pet.tags {
                for tag in tags {
                    let _: () = conn.srem(format!("pet_tag:{}", tag.name), id)?;
                }
            }
        }
        let _: () = conn.hdel("pets", id)?;
        Ok(())
    }

    pub fn get_pet_by_name(&mut self, name: &str) -> redis::RedisResult<Option<Pet>> {

        // get connection from pool of
        let mut conn = self.client.get_connection().expect("Failed to get connection");
        

        let id: Option<u64> = conn.hget("pet_names", name)?;
        id.map_or(Ok(None), |id| self.get_pet_by_id(id))
    }

    pub fn get_pets_by_status(&mut self, status: &str) -> redis::RedisResult<Vec<Pet>> {

        // get connection from pool of 
        let mut conn = self.client.get_connection().expect("Failed to get connection");

        let status: Vec<&str> = status.split(',').collect();
        let mut pets: Vec<Pet> = vec![];
        for s in status {
            let status_key = format!("pet_status:{}", s);
            let ids: Vec<u64> = conn.smembers(status_key)?;
            for id in ids {
                if let Some(pet) = self.get_pet_by_id(id)? {
                    pets.push(pet);
                }
            }
        }
        Ok(pets)
    }

    pub fn get_pets_by_tags(&mut self, tag: &str) -> redis::RedisResult<Vec<Pet>> {

        // get connection from pool of 
        let mut conn = self.client.get_connection().expect("Failed to get connection");

        let tag: Vec<&str> = tag.split(',').collect();
        let mut pets: Vec<Pet> = vec![];
        for t in tag {
            let tag_key = format!("pet_tag:{}", t);
            let ids: Vec<u64> = conn.smembers(tag_key)?;
            for id in ids {
                if let Some(pet) = self.get_pet_by_id(id)? {
                    pets.push(pet);
                }
            }
        }
        Ok(pets)
    }

    pub fn add_user(&mut self, user: &User) -> redis::RedisResult<()> {
      
        // get connection from pool of 
        let mut conn = self.client.get_connection().expect("Failed to get connection");

        let user_json = Self::serialize(user).map_err(|e| {
            error!("Serialization error: {}", e);
            redis::RedisError::from((redis::ErrorKind::TypeError, "Serialization error"))
        })?;
        let _: () = conn.hset("users", user.id, user_json)?;
        let _: () = conn.hset("user_names", user.username.clone(), user.id)?;
        Ok(())
    }

    pub fn get_users(&mut self) -> redis::RedisResult<Vec<User>> {

        // get connection from pool of
        let mut conn = self.client.get_connection().expect("Failed to get connection");

        let user_map: std::collections::HashMap<String, String> = conn.hgetall("users")?;
        let users: Vec<User> = user_map
            .values()
            .filter_map(|json| Self::deserialize(json).ok())
            .collect();
        Ok(users)
    }

    pub fn get_user_by_id(&mut self, id: u64) -> redis::RedisResult<Option<User>> {
        
        // get connection from pool of
        let mut conn = self.client.get_connection().expect("Failed to get connection");

        let user_json: Option<String> = conn.hget("users", id)?;
        user_json.map_or(Ok(None), |json| {
            Self::deserialize(&json).map(Some).map_err(|e| {
                error!("Deserialization error: {}", e);
                redis::RedisError::from((redis::ErrorKind::TypeError, "Deserialization error"))
            })
        })
    }

    pub fn get_user(&mut self, username: &str) -> redis::RedisResult<Option<User>> {
        // get connection from pool of
        let mut conn = self.client.get_connection().expect("Failed to get connection");

        let id: Option<u64> = conn.hget("user_names", username)?;
        id.map_or(Ok(None), |id| self.get_user_by_id(id))
    }

    pub fn update_user(&mut self, user: &User) -> redis::RedisResult<()> {
        if let Some(my_user) = self.get_user_by_id(user.id)? {
            self.delete_user(my_user.id)?;
        }
        self.add_user(user)
    }

    pub fn delete_user(&mut self, id: u64) -> redis::RedisResult<()> {

        // Get connection from pool of
        let mut conn = self.client.get_connection().expect("Failed to get connection");

        if let Some(user) = self.get_user_by_id(id)? {
            let _: () = conn.hdel("user_names", user.username)?;
        }
        let _: () = conn.hdel("users", id)?;
        Ok(())
    }

    pub fn delete_user_by_username(&mut self, username: &str) -> redis::RedisResult<()> {
        
        // get connection from pool of
        let mut conn = self.client.get_connection().expect("Failed to get connection");
        
        let id: Option<u64> = conn.hget("user_names", username)?;
        id.map_or(Ok(()), |id| self.delete_user(id))
    }
}
