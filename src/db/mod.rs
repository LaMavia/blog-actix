pub mod models;

use postgres::{Connection, TlsMode};

pub fn connect_to_db() -> Connection {
	Connection::connect(
		"postgresql://tomasz:toor@localhost:5432/blog-actix",
		TlsMode::None,
	)
	.unwrap()
}