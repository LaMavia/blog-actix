use crate::db::{connect_to_db, models};
use actix_web;
use actix_web::HttpResponse;

#[get("api/posts")]
pub fn all() -> HttpResponse {
  let conn = connect_to_db();
  let mut posts = vec![];

  // -------- Warning for a future me -------- //
  // It's not really efficent, but I don't expect any traction on the website, so...
  for p in conn.query("SELECT posts.id, users.login, posts.date, posts.title, posts.body FROM posts INNER JOIN users ON posts.author = users.id", &[]).unwrap().into_iter()
  {
    posts.push(models::QueriedPost::new(p.get(0), p.get(1), p.get(2), p.get(3), p.get(4)));
  };

  HttpResponse::Ok().body(json!(&posts).to_string())
}