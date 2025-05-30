CREATE TABLE user_trip (
  user_id UUID NOT NULL,
  trip_id UUID NOT NULL,
  permission TEXT,

  PRIMARY KEY (user_id, trip_id),
  FOREIGN KEY (user_id)
    REFERENCES users(id),
  FOREIGN KEY (trip_id)
    REFERENCES trips(id)
    ON DELETE CASCADE
);
