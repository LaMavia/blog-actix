#![allow(dead_code)]
extern crate chrono;
extern crate postgres;
use serde::{Deserialize, Serialize};

pub trait Model {
  fn insert() -> &'static str;
}
#[derive(Serialize, Deserialize, Clone)]
pub struct User {
  id: i32,
  login: String,
  password: String,
}

impl Model for User {
  fn insert() -> &'static str {
    "INSERT INTO users (login, password) VALUES ($1, $2)"
  }
}

impl User {
  pub fn new(id: i32, login: String) -> User {
    User {
      id,
      login,
      password: String::new(),
    }
  }
}

pub struct Post {
  id: i32,
  author: i32,
  date: String,
  title: String,
  body: String,
}

impl Model for Post {
  fn insert() -> &'static str {
    "INSERT INTO posts (id, author, date, title, body) VALUES ($1, $2, $3, $4, $5)"
  }
}

impl Post {
  pub fn new(id: i32, author: i32, date: String, title: String, body: String) -> Post {
    Post {
      id,
      author,
      date,
      title,
      body,
    }
  }

  pub fn insert_wo_id() -> &'static str {
    "INSERT INTO posts (author, date, title, body) VALUES ($1, $2, $3, $4)"
  }

  pub fn id(&self) -> i32 {
    self.id
  }
  pub fn author(&self) -> i32 {
    self.author
  }
  pub fn date(&self) -> String {
    self.date.to_owned()
  }
  pub fn title(&self) -> String {
    self.title.to_owned()
  }
  pub fn body(&self) -> String {
    self.body.to_owned()
  }

  fn q_by_user() -> String {
    String::from("SELECT id, author, date, title, body FROM post WHERE author=$1")
  }
}
