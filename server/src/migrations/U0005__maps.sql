CREATE TABLE maps (
  id UUID PRIMARY KEY,
  user_id UUID NOT NULL REFERENCES users,
  map_type TEXT,
  title TEXT NOT NULL
);
