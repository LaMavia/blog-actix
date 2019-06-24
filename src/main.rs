#[macro_use]
extern crate actix_web;
extern crate actix_files as fs;

#[macro_use]
extern crate serde_json;

use actix_web::{middleware, App, HttpServer};
use std::io;
mod db;
mod routes;

fn main() -> io::Result<()> {
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
