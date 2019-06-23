#[macro_use]
extern crate actix_web;
use actix_files as fs;

extern crate postgres;

use actix_web::web;
use actix_web::{middleware, App, HttpResponse, HttpServer};

use std::io;
use std::sync::Arc;

use postgres::{Connection, TlsMode};

mod models;
use models::{Post, User};

type DatabaseData<T> = Arc<Vec<T>>;
type WebDb<T> = web::Data<DatabaseData<T>>;

// Macro documentation can be found in the actix_web_codegen crate
#[get("*")]
fn index(users: WebDb<User>, posts: WebDb<Post>) -> HttpResponse {
	HttpResponse::Ok().body(String::from(
		"<!DOCTYPE html>
			<html lang=\"en\">
			
			<head>
				<meta charset=\"UTF-8\">
				<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
				<meta http-equiv=\"X-UA-Compatible\" content=\"ie=edge\">
				<title>Document</title>
				<link rel=\"stylesheet\" href=\"/static/css/styles.css\">
			</head>
			<body>
			<div id=\"root\"></div>
			</body>
			<script src=\"/js/Index.js\"></script>
			</html>",
	))
}

fn get_users(conn: &Connection) -> DatabaseData<User> {
	let mut users: Vec<User> = vec![];

	for u in conn
		.query("SELECT id, login FROM users", &[])
		.unwrap()
		.into_iter()
	{
		users.push(User::new(u.get(0), u.get(1)));
	}

	Arc::new(users)
}

fn get_posts(conn: &Connection) -> DatabaseData<Post> {
	let mut posts: Vec<Post> = vec![];

	for p in conn
		.query("SELECT id, author, date, title, body FROM posts", &[])
		.unwrap()
		.into_iter()
	{
		posts.push(Post::new(p.get(0), p.get(1), p.get(2), p.get(3), p.get(4)));
	}

	Arc::new(posts)
}

fn connect_to_db() -> Connection {
	Connection::connect(
		"postgresql://tomasz:toor@localhost:5432/blog-actix",
		TlsMode::None,
	)
	.unwrap()
}

fn main() -> io::Result<()> {
	// -------- Logger -------- //
	std::env::set_var("RUST_LOG", "actix_web=info");
	env_logger::init();

	// -------- PostgreSQL connection -------- //
	let conn = connect_to_db();

	let p = Post::new(
		0,
		0,
		"2019-06-21".to_string(),
		"Cats will rule the world".to_string(),
		"They sure will. \nThe question is - when?".to_string(),
	);

	for _ in conn
		.query(
			Post::insert_wo_id(),
			&[&p.author(), &p.date(), &p.title(), &p.body()],
		)
		.unwrap()
		.into_iter()
	{}

	let users_ref = web::Data::new(get_users(&conn));
	let posts_ref = web::Data::new(get_posts(&conn));

	HttpServer::new(move || {
		App::new()
			.register_data(users_ref.clone())
			.register_data(posts_ref.clone())
			.wrap(middleware::Logger::new("%s %U %a"))
			.service(fs::Files::new("/static", "./static"))
			.service(fs::Files::new("/js", "./client/build"))
			.service(index)
	})
	.bind("127.0.0.1:8000")?
	.run()
}
