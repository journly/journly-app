CREATE TABLE user_map (
  user_id UUID NOT NULL,
  map_id UUID NOT NULL,

  PRIMARY KEY (user_id, map_id),
  FOREIGN KEY (user_id)
    REFERENCES users(id),
  FOREIGN KEY (map_id)
    REFERENCES maps(id)
    ON DELETE CASCADE
);
