#[macro_use]
extern crate actix_web;
extern crate actix_files as fs;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{middleware, App, HttpServer};
use std::io;
mod db;
mod routes;

use dotenv::dotenv;

fn main() -> io::Result<()> {
	// -------- Check for / load DATABASE_URL $env -------- //
	match std::env::var("DATABASE_URL").ok() {
		Some(_) => {}
		None => {
			dotenv().ok();
			();
		}
	}
	// -------- Logger -------- //
	std::env::set_var("RUST_LOG", "actix_web=info");
	env_logger::init();

	// -------- Server -------- //
	HttpServer::new(move || {
		App::new()
			// .register_data(Data::new(queries))
			.wrap(middleware::Logger::new("%s %U %a"))
			.service(fs::Files::new("/static", "./static"))
			.service(fs::Files::new("/js", "./client/build"))
			.service(routes::post::all)
			.service(routes::index::route)
	})
	.bind("127.0.0.1:8000")?
	.run()
}
