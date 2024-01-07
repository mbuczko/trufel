CREATE TABLE IF NOT EXISTS users
(
  user_id UUID PRIMARY KEY,
  email TEXT NOT NULL,
  name TEXT NOT NULL,
  picture TEXT,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS applications
(
  application_id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT,
  url TEXT NOT NULL,
  icon TEXT,
  visible BOOLEAN NOT NULL DEFAULT TRUE,
  order: INTEGER NOT NULL
);

CREATE UNIQUE INDEX users_email_idx ON users(email);
