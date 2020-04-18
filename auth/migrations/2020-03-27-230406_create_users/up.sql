-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR(100) NOT NULL,
  password VARCHAR(64) NOT NULL,
  created_at TIMESTAMP NOT NULL
);