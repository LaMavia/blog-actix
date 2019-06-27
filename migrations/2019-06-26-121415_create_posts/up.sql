CREATE TABLE users (
  id SERIAL NOT NULL,
	login VARCHAR NOT NULL,
	password VARCHAR NOT NULL,
  primary key (id)
);

CREATE TABLE posts (
  id SERIAL NOT NULL,
  author int NOT NULL,
  date VARCHAR NOT NULL,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  primary key (id),

  foreign key (author) references users(id)
);

