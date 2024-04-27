// usermodel.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)] // Added `Clone` trait derivation
pub struct User {
	pub id: u64,
	pub username: String,
	pub email: String,
	pub password: String,
}
