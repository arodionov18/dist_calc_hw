CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR(100) NOT NULL,
  password VARCHAR(64) NOT NULL,
  created_at TIMESTAMP NOT NULL,
  confirmed INT DEFAULT 0 NOT NULL
);

CREATE TABLE session (
  id SERIAL PRIMARY KEY,
  refresh_token TEXT NOT NULL,
  refresh_token_expire_at TIMESTAMP NOT NULL,
  user_id SERIAL NOT NULL REFERENCES users (id)
);