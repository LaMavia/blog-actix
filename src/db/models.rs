#[allow(dead_code)]
use super::schema::{posts, users};
use serde::{Deserialize, Serialize};
use diesel::{QueryDsl, RunQueryDsl};

#[derive(Queryable, QueryableByName, Serialize, Deserialize)]
#[table_name="posts"]
pub struct Post {
  pub id: i32,
  pub author: i32,
  pub date: String,
  pub title: String,
  pub body: String,
}

#[derive(Queryable, QueryableByName, Serialize, Deserialize)]
#[table_name="users"]
pub struct User {
  pub id: i32,
  pub login: String,
  pub password: String,
}

type DieselResult<T> = Result<T, diesel::result::Error>;

impl Post {
  pub fn with_users(c: &diesel::PgConnection) -> DieselResult<Vec<(Post, User)>> {
    posts::table.inner_join(users::table).load(c) 
  }
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
  pub author: i32,
  pub date: &'a str,
  pub title: &'a str,
  pub body: &'a str,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
  pub login: &'a str,
  pub password: &'a str,
}


#[derive(Serialize, Deserialize)]
pub struct JsonyPost {
  pub id: i32,
  pub author: String,
  pub date: String,
  pub title: String,
  pub body: String
}

impl JsonyPost {
  pub fn new<'a>(id: i32, author: &'a str, date: &'a str, title: &'a str, body: &'a str) -> JsonyPost {
    JsonyPost {
      id,
      author: author.to_string(),
      date: date.to_string(),
      title: title.to_string(),
      body: body.to_string()
    }
  }
}