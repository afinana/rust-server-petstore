// petmodel.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pet {
    pub id: u64,
    pub category: Category,
    pub name: String,
    #[serde(rename = "photoUrls", default)]
    pub photo_urls: Vec<String>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize,Clone)] // Added `Clone` trait derivation
pub struct Category {
    pub	id: u64,
	pub	name: String
}

impl Category {}   
#[derive(Debug, Serialize, Deserialize,Clone)] // Added `Clone` trait derivation
pub struct Tag {
	pub id: u64,
	pub name: String
}

