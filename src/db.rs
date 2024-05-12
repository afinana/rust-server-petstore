// db.rs


use redis::Commands;
use crate::petmodel::Pet;

pub struct RedisDb {
   pub client: redis::Connection,
}

impl RedisDb {


    pub fn add_pet(&mut self, pet: &Pet) -> redis::RedisResult<()> {
        let pet_json = serde_json::to_string(pet).unwrap();
        let _: () = self.client.hset("pets", pet.id, pet_json)?;
        // save pet name indexed by name
        let _: () = self.client.hset("pet_names", pet.name.clone(), pet.id)?;
       
		// create string as pet_status:+status
		let status_key = format!("pet_status:{}", pet.status.clone().unwrap_or("".to_string()));
		 
		
		 // add pet id to a set of pet_status:+status
		 let _: () = self.client.sadd(status_key, pet.id)?;
		
        // save pet tags indexed by tag name
        if let Some(tags) = &pet.tags {
			for tag in tags {
				// add pet id to a set of pet_tags:+tag_name
				let tag_key = format!("pet_tag:{}", tag.name.clone());
				let _: () = self.client.sadd(tag_key, pet.id)?;
				
			}
		}

		Ok(())
     
    }
	// update pet
	pub fn update_pet(&mut self, pet: &Pet) -> redis::RedisResult<()> {
		// remove pet name indexed by name
        let my_pet: Option<Pet> = self.get_pet_by_id(pet.id)?;
		// if pet exists call delete_pet
		if let Some(my_pet) = my_pet {
			self.delete_pet(my_pet.id)?;
		}

		// call update_pet and return Result
		self.add_pet(pet)
	}

	// update pet by id
	pub fn update_pet_by_id(&mut self, id: u64, pet: &Pet) -> redis::RedisResult<()> {
		// add log pet.id
		log::info!("update_pet_by_id: {}", id);
		// update pet by id and return Result
		self.add_pet(pet)
		
		 
	}// end fn
			

    pub fn get_pets(&mut self) -> redis::RedisResult<Vec<Pet>> {
		// get all pets
        let pet_map: std::collections::HashMap<String, String> = self.client.hgetall("pets")?;
        // convert pet json to pet struct
		let pets: Vec<Pet> = pet_map.values().map(|json| serde_json::from_str(json).unwrap()).collect();
        Ok(pets)
    }// end fn

    pub fn get_pet_by_id(&mut self, id: u64) -> redis::RedisResult<Option<Pet>> {
		// get pet by id
	    let pet_json: Option<String> = self.client.hget("pets", id)?;
        // convert pet json to pet struct
		let pet: Option<Pet> = match pet_json {
            Some(json) => Some(serde_json::from_str(&json).unwrap()),
            None => None,
        };
        Ok(pet)
    }// end fn

    pub fn delete_pet(&mut self, id: u64) -> redis::RedisResult<()> {

        // remove pet name indexed by name
        let pet: Option<Pet> = self.get_pet_by_id(id)?;		
        // if pet exists
		if let Some(pet) = pet {
			// remove pet name indexed by name
			let _: () = self.client.hdel("pet_names", pet.name)?;
			 // remove pet id to a set of pet_status:+status
			 let _: () = self.client.srem(format!("pet_status:{}", pet.status.unwrap_or("".to_string())), id)?;
			 // remove pet id to a set of pet_tags:+tag_name
			 if let Some(tags) = pet.tags {
				 for tag in tags {
					let _: () = self.client.srem(format!("pet_tag:{}", tag.name), id)?;
				}// end for
			}// end if

		}// end if
		
		// remove pet category indexed by category name			
		let _: () = self.client.hdel("pets", id)?;		
        Ok(())
    }// end fn

    // search pet by name
    pub fn get_pet_by_name(&mut self, name: &str) -> redis::RedisResult<Option<Pet>> {
		// get pet id by name
		let id: Option<u64> = self.client.hget("pet_names", name)?;
		// get pet by id
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
      // for each status
	  for s in status {
		// create string as pet_status:+status
		let status_key = format!("pet_status:{}", s);
		
		// find pet id of set status
		let ids: Vec<u64> = self.client.smembers(status_key)?;
		// for each pet id, get pet
		for id in ids {
			 // get pet by id
			let pet = self.get_pet_by_id(id)?;
			// if pet exists, add to pets
			match pet {
				Some(pet) => pets.push(pet),
				None => (),
			}
		}// end for
	  }// end for
	  Ok(pets)
	}// end fn
	
    // search pet by tag
    pub fn get_pet_by_tags(&mut self, tag: &str) -> redis::RedisResult<Vec<Pet>> {
        
        // parse a string of elements sparated by comma
        let tag: Vec<&str> = tag.split(',').collect();
        let mut pets: Vec<Pet> = vec![];
        // for each tag
		for t in tag {
		    
			// create string as pet_tag:+tag
			let tag_key = format!("pet_tag:{}", t);
			// find pet id of set tag
			let ids: Vec<u64> = self.client.smembers(tag_key)?;
			// for each pet id, get pet
			for id in ids {
				let pet = self.get_pet_by_id(id)?;
				match pet {
					Some(pet) => pets.push(pet),
					None => (),
			    }
			} // end for
		}// end for
		Ok(pets)
	}// end fn
}// end impl

	


	
	

