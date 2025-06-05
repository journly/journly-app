CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  username TEXT NOT NULL,
  email TEXT UNIQUE NOT NULL,
  password_hash TEXT,
  password_salt BYTEA,
  avatar TEXT,
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
