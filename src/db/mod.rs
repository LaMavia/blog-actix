use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;

pub mod models;
pub mod schema;
pub mod actions;

pub fn connect_to_db() -> PgConnection {
	let database_url = env::var("DATABASE_URL")
		.expect("DATABASE_URL must be set");

	PgConnection::establish(&database_url)
		.expect(&format!("Error connecting to {}", database_url))	
}