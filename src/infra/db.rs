use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
	dotenv().ok();
	let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	
	std::fs::create_dir_all("database").ok();

	SqliteConnection::establish(&db_url)
		.unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}