#[macro_use]
extern crate actix_web;
use actix_files as fs;
#[macro_use]
extern crate serde_json;

extern crate postgres;

use actix_web::web;
use actix_web::{middleware, App, HttpResponse, HttpServer};

use handlebars::Handlebars;

use std::io;
use std::sync::Arc;

use postgres::{Connection, TlsMode};

mod models;
use models::{Post, User};

type DatabaseData<T> = Arc<Vec<T>>;
type WebDb<T> = web::Data<DatabaseData<T>>;

// Macro documentation can be found in the actix_web_codegen crate
#[get("/")]
fn index(hb: web::Data<Arc<Handlebars>>, users: WebDb<User>) -> HttpResponse {
	let mut data = json!({
		"users": &users.to_vec()
	});

	let body = hb.render("index", &data).unwrap();
	HttpResponse::Ok().body(body)
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

fn get_posts(conn: &Connection) -> Vec<Post> {
	let mut posts: Vec<Post> = vec![];

	for p in conn
		.query("SELECT id, author, date, title, body FROM posts", &[])
		.unwrap()
		.into_iter()
	{
		posts.push(Post::new(p.get(0), p.get(1), p.get(2), p.get(3), p.get(4)));
	}

	posts
}

fn connect_to_db() -> Connection {
	Connection::connect(
		"postgresql://tomasz:toor@localhost:5432/blog-actix",
		TlsMode::None,
	)
	.unwrap()
}

fn main() -> io::Result<()> {
	// Setup the Handlebars
	let mut handlebars = Handlebars::new();
	handlebars
		.register_templates_directory(".hbs", "./routes")
		.unwrap();
	let handlebars_ref = web::Data::new(Arc::new(handlebars));

	// Setup the Logger
	std::env::set_var("RUST_LOG", "actix_web=info");
	env_logger::init();

	// Setup the PostgreSQL connection
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
			.register_data(handlebars_ref.clone())
			.register_data(users_ref.clone())
			.register_data(posts_ref.clone())
			.wrap(middleware::Logger::new("%s %U %a"))
			.service(index)
			.service(fs::Files::new("/static", "./static"))
	})
	.bind("127.0.0.1:8000")?
	.run()
}
