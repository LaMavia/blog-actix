use crate::db;
use actix_web;
use actix_web::HttpResponse;
use db::models::JsonyPost;

#[get("api/posts")]
pub fn all() -> HttpResponse {
  let conn = db::connect_to_db();
  let posts = db::actions::get_posts(&conn);
  let mut ps: Vec<JsonyPost> = vec![];

  for (p, u) in posts.iter().as_ref() {
    ps.push(JsonyPost::new(
      p.id, &*u.login, &*p.date, &*p.title, &*p.body,
    ));
  }

  HttpResponse::Ok().body(json!(ps).to_string())
}
