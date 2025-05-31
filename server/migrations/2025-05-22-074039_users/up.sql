CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  username TEXT NOT NULL,
  email TEXT UNIQUE NOT NULL,
  password_hash TEXT NOT NULL,
  password_salt BYTEA NOT NULL,
  avatar TEXT,
  is_admin BOOL NOT NULL DEFAULT FALSE
);
