// petmodel.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)] // Added `Clone` trait derivation
pub struct Pet {
     
    pub id: u64, 
    // add category field as struct with id and name
    pub category: Category,    
   
    pub name: String, 
    // Added `status` field
    pub photo_urls: Option<Vec<String>>,   
    pub tags: Option<Vec<Tag>>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)] // Added `Clone` trait derivation
pub struct Category {
    pub	id: u64,
	pub	name: String
}

impl Category {}   
#[derive(Debug, Serialize, Deserialize, Clone)] // Added `Clone` trait derivation
pub struct Tag {
	pub id: u64,
	pub name: String
}

// functions to create new instances of structs
impl Pet {
	pub fn new(id: u64, category: Category, name: String) -> Self {
		Pet {
			id,
			category,		
			name,
			photo_urls: None,
			status: None,
			tags: None
		}
	}
}
