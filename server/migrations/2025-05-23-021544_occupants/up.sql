CREATE TABLE occupants (
  accommodation_id UUID NOT NULL REFERENCES accommodations(id) ON DELETE CASCADE,
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

  PRIMARY KEY (accommodation_id, user_id)
);
