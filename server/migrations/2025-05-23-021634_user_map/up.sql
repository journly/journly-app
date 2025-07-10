CREATE TABLE user_map (
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  map_id UUID NOT NULL REFERENCES maps(id) ON DELETE CASCADE,

  PRIMARY KEY (user_id, map_id)
);
