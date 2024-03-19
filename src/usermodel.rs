// usermodel.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)] // Added `Clone` trait derivation
pub struct User {
	pub id: u64,
	pub username: String,
	pub email: String,
	pub password: String,
}
// add new function to create new instance of User
impl User {
	pub fn new(id: u64, username: String, email: String, password: String) -> Self {
		User {
			id,
			username,
			email,
			password
		}
	}
}
