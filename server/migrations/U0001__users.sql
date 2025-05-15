DROP TABLE IF EXISTS users;

CREATE TABLE users (
  id UUID PRIMARY KEY,
  display_name TEXT,
  username TEXT NOT NULL UNIQUE,
  email TEXT UNIQUE,
  password_hash TEXT NOT NULL,
  password_salt BYTEA NOT NULL,
  profile_picture_url TEXT
);

INSERT INTO users(id, username, password_hash, password_salt)
VALUES ('612e21ed-869b-4130-bb72-fc7549f93609', 'test_user1', 'test_password', '\001'::bytea);

INSERT INTO users(id, username, password_hash, password_salt)
VALUES ('3b918c91-0cf2-4788-93f9-10d1f77ec3a9', 'test_user2', 'test_password', '\001'::bytea);

INSERT INTO users(id, username, password_hash, password_salt)
VALUES (gen_random_uuid(), 'johnharthone', 'password', '\001'::bytea);

INSERT INTO users(id, username, password_hash, password_salt)
VALUES (gen_random_uuid(), 'tomfoolery', 'password', '\001'::bytea);

INSERT INTO users(id, username, password_hash, password_salt)
VALUES (gen_random_uuid(), 'monkeyman', 'password', '\001'::bytea);
