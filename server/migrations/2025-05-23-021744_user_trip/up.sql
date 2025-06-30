CREATE TABLE user_trip (
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  trip_id UUID NOT NULL REFERENCES trips(id) ON DELETE CASCADE,
  permission TEXT,

  PRIMARY KEY (user_id, trip_id)
);
