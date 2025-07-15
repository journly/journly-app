CREATE TABLE passengers (
  flight_id UUID NOT NULL REFERENCES flights(id) ON DELETE CASCADE,
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

  PRIMARY KEY (flight_id, user_id)
);
