CREATE TABLE passengers (
  flight_id UUID NOT NULL,
  user_id UUID NOT NULL,

  PRIMARY KEY (flight_id, user_id),
  FOREIGN KEY (flight_id)
    REFERENCES flights(id)
    ON DELETE CASCADE,
  FOREIGN KEY (user_id)
    REFERENCES users(id)
    ON DELETE CASCADE
);
