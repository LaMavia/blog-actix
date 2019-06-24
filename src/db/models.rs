#![allow(dead_code)]
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

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Hash,Debug,Serialize,Deserialize)]
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
}

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Hash,Debug,Serialize,Deserialize)]
pub struct QueriedPost {
  id: i32,
  author: String,
  date: String,
  title: String,
  body: String,
}

impl QueriedPost {
  pub fn new(id: i32, author: String, date: String, title: String, body: String) -> QueriedPost {
    QueriedPost {
      id,
      author,
      date,
      title,
      body,
    }
  }
}
