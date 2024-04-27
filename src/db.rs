// db.rs

use redis::Commands;
use crate::petmodel::Pet;

pub struct RedisDb {
   pub client: redis::Connection,
}

impl RedisDb {
    pub fn new(redis_url: &str) -> redis::RedisResult<Self> {
        let client = redis::Client::open(redis_url)?.get_connection()?;
        Ok(RedisDb { client })
    }
    

    pub fn add_pet(&mut self, pet: &Pet) -> redis::RedisResult<()> {
        let pet_json = serde_json::to_string(pet).unwrap();
        let _: () = self.client.hset("pets", pet.id, pet_json)?;
        // save pet name indexed by name
        let _: () = self.client.hset("pet_names", pet.name.clone(), pet.id)?;
        // save  pet category indexed by category name
        let _: () = self.client.hset("pet_categories", pet.category.name.clone(), pet.id)?;

        
        // save pet status indexed by status
        let _: () = self.client.hset("pet_statuses", pet.status.clone().unwrap_or("".to_string()), pet.id)?;

        // save pet tags indexed by tag name
        if let Some(tags) = &pet.tags {
			for tag in tags {
				let _: () = self.client.hset("pet_tags", tag.name.clone(), pet.id)?;
			}
		}

		Ok(())
     
    }

    pub fn get_pets(&mut self) -> redis::RedisResult<Vec<Pet>> {
        let pet_map: std::collections::HashMap<String, String> = self.client.hgetall("pets")?;
        let pets: Vec<Pet> = pet_map.values().map(|json| serde_json::from_str(json).unwrap()).collect();
        Ok(pets)
    }

    pub fn get_pet_by_id(&mut self, id: u64) -> redis::RedisResult<Option<Pet>> {
        let pet_json: Option<String> = self.client.hget("pets", id)?;
        let pet: Option<Pet> = match pet_json {
            Some(json) => Some(serde_json::from_str(&json).unwrap()),
            None => None,
        };
        Ok(pet)
    }

    pub fn delete_pet(&mut self, id: u64) -> redis::RedisResult<()> {

        let _: () = self.client.hdel("pets", id)?;
        // remove pet name indexed by name
        let pet: Option<Pet> = self.get_pet_by_id(id)?;
        if let Some(pet) = pet {
			let _: () = self.client.hdel("pet_names", pet.name)?;
			// remove pet category indexed by category name
			let _: () = self.client.hdel("pet_categories", pet.category.name)?;
			// remove pet status indexed by status
			let _: () = self.client.hdel("pet_statuses", pet.status.unwrap_or("".to_string()))?;
			// remove pet tags indexed by tag name
			if let Some(tags) = pet.tags {
				for tag in tags {
					let _: () = self.client.hdel("pet_tags", tag.name)?;
				}
			}
		}

        Ok(())
    }
    // search pet by name
    pub fn get_pet_by_name(&mut self, name: &str) -> redis::RedisResult<Option<Pet>> {
		let id: Option<u64> = self.client.hget("pet_names", name)?;
		match id {
			Some(id) => self.get_pet_by_id(id),
			None => Ok(None),
		}
	}
   
	// search pet by status
    pub fn get_pets_by_status(&mut self, status: &str) -> redis::RedisResult<Vec<Pet>> {
      // parse a string of elements sparated by comma
      let status: Vec<&str> = status.split(',').collect();
      let mut pets: Vec<Pet> = vec![];
      for s in status {
		  let id: Option<u64> = self.client.hget("pet_statuses", s)?;
		  match id {
			  Some(id) => {
				  let pet = self.get_pet_by_id(id)?;
				  match pet {
					  Some(pet) => pets.push(pet),
					  None => (),
				  }
			  }
			  None => (),
		  }
	  }
      Ok(pets)
	}
	// search pet by category
	pub fn get_pets_by_category(&mut self, category: &str) -> redis::RedisResult<Vec<Pet>> {
		let id: Option<u64> = self.client.hget("pet_categories", category)?;
		match id {
			Some(id) => {
				let pet = self.get_pet_by_id(id)?;
				match pet {
					Some(pet) => Ok(vec![pet]),
					None => Ok(vec![]),
				}
			}
			None => Ok(vec![]),
		}
      
	}
    // search pet by tag
    pub fn get_pet_by_tags(&mut self, tag: &str) -> redis::RedisResult<Vec<Pet>> {
        
        // parse a string of elements sparated by comma
        let tag: Vec<&str> = tag.split(',').collect();
        let mut pets: Vec<Pet> = vec![];
        for t in tag {
			let id: Option<u64> = self.client.hget("pet_tags", t)?;
			match id {
				Some(id) => {
					let pet = self.get_pet_by_id(id)?;
					match pet {
						Some(pet) => pets.push(pet),
						None => (),
					}
				}
				None => (),
			}
		}
		Ok(pets)
    }
    
    

}
