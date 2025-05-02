CREATE TABLE users (
  id UUID PRIMARY KEY,
  display_name TEXT, 
  username TEXT NOT NULL UNIQUE,
  email TEXT UNIQUE,
  password_hash TEXT NOT NULL,
  profile_picture_url TEXT
);
