// petmodel.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)] // Added `Clone` trait derivation
pub struct Pet {
     
    pub id: u64, 
    // add category field as struct with id and name
    pub category: Category,    
   
    pub name: String, 
    // add photoUrls as array of strings   
    pub photoUrls: Vec<String>,
    
    pub tags: Option<Vec<Tag>>,
    pub status: String,
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

