CREATE TABLE IF NOT EXISTS users
(
  user_id UUID PRIMARY KEY,
  email TEXT NOT NULL,
  name TEXT NOT NULL,
  picture TEXT,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS categories
(
  category_id UUID PRIMARY KEY,
  user_id UUID NOT NULL,
  name TEXT NOT NULL,
  position INTEGER NOT NULL DEFAULT 0,

  FOREIGN KEY (user_id) REFERENCES users(user_id)
);

CREATE TABLE IF NOT EXISTS applications
(
  application_id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT,
  url TEXT NOT NULL,
  icon TEXT,
  visibility BOOLEAN NOT NULL DEFAULT TRUE,
  position INTEGER NOT NULL,
  user_id UUID NOT NULL,

  FOREIGN KEY (user_id) REFERENCES users(user_id)
);


CREATE TABLE IF NOT EXISTS bookmarks
(
  bookmark_id UUID PRIMARY KEY,
  category_id UUID NOT NULL,
  name TEXT NOT NULL,
  url TEXT NOT NULL,
  icon TEXT,
  visibility BOOLEAN NOT NULL DEFAULT TRUE,
  position INTEGER NOT NULL DEFAULT 0,
  user_id UUID NOT NULL,

  FOREIGN KEY (category_id) REFERENCES categories(category_id)
  FOREIGN KEY (user_id) REFERENCES users(user_id)
);

CREATE UNIQUE INDEX users_email_idx ON users(email);
