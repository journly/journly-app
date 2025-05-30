CREATE TABLE trip_invites (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  trip_id UUID NOT NULL,
  user_id UUID,
  invitee_email TEXT,
  status TEXT DEFAULT 'pending',
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,

  FOREIGN KEY (trip_id)
    REFERENCES trips(id)
    ON DELETE CASCADE,
  FOREIGN KEY (user_id)
    REFERENCES users(id)
);
