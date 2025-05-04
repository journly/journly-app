DROP TABLE IF EXISTS users;

CREATE TABLE users (
  id UUID PRIMARY KEY,
  display_name TEXT, 
  username TEXT NOT NULL UNIQUE,
  email TEXT UNIQUE,
  password_hash TEXT NOT NULL,
  profile_picture_url TEXT
);


INSERT INTO users(id, username, password_hash) 
VALUES (gen_random_uuid(), 'johnharthone', 'password');

INSERT INTO users(id, username, password_hash) 
VALUES (gen_random_uuid(), 'tomfoolery', 'password');

INSERT INTO users(id, username, password_hash)
VALUES (gen_random_uuid(), 'monkeyman', 'password');
