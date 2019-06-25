create table posts (
  id int not null primary key,
  author int not null,
  title varchar(255),
  date varchar(255),
  body text
);

create table users (
  id int not null primary key,
  login varchar(255),
  password varchar(255),
);