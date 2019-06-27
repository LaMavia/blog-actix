use super::models::{Post, NewPost, User, NewUser};
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn create_post<'a>(conn: &PgConnection, author: i32, date: &'a str, title: &'a str, body: &'a str) -> Post {
  use super::schema::posts;

  let new_post = NewPost {
    author: author,
    date: date,
    title: title,
    body: body,
  };

  diesel::insert_into(posts::table)
    .values(&new_post)
    .get_result(conn)
    .expect("Error inserting post")
}

pub fn get_posts<'a>(conn: &PgConnection) -> Vec<(Post, User)> {
  // .select((id, author, date, title, body))
  
  let ps = Post::with_users(conn).expect("Error querying posts (inner join users)");
  ps
}