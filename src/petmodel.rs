// petmodel.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pet {
    pub id: i64,
    pub category: Category,
    pub name: String,
    #[serde(rename = "photoUrls", default)]
    pub photo_urls: Vec<String>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}
