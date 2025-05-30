CREATE TABLE occupants (
  accommodation_id UUID NOT NULL,
  user_id UUID NOT NULL,

  PRIMARY KEY (accommodation_id, user_id),
  FOREIGN KEY (accommodation_id)
    REFERENCES accommodations(id)
    ON DELETE CASCADE,
  FOREIGN KEY (user_id)
    REFERENCES users(id)
    ON DELETE CASCADE
);
