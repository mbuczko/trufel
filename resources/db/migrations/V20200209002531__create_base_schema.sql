CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;

CREATE TABLE IF NOT EXISTS users
(
  user_id UUID PRIMARY KEY,
  sub VARCHAR(64) NOT NULL,
  email VARCHAR(255) NOT NULL,
  name VARCHAR(255) NOT NULL,
  picture VARCHAR(1024),
  created_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX users_sub_idx ON users(sub);
CREATE UNIQUE INDEX users_email_idx ON users(email);
