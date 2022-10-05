CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;

CREATE TABLE IF NOT EXISTS users
(
  user_id UUID PRIMARY KEY,
  email VARCHAR(255) NOT NULL,
  idp_name VARCHAR(255) NOT NULL,
  idp_picture VARCHAR(1024),
  created_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX users_email_idx ON users(email);
