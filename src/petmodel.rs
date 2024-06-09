// petmodel.rs

use amqprs::channel::Channel;
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
// create appdata struct to hold redis_db and channel
pub struct AppData {
	pub redis_db: web::Data<Mutex<db::RedisDb>>,
	pub mq: web::Data<Mutex<RabbitMQ>>,
}


impl AppData {
    pub fn new(redis_db: web::Data<Mutex<db::RedisDb>>, mq: web::Data<Mutex<RabbitMQ>>) -> Self {
        AppData { redis_db, mq }
    }
}
