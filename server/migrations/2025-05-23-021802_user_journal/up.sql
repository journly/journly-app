CREATE TABLE user_journal (
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  journal_id UUID NOT NULL REFERENCES journals(id) ON DELETE CASCADE,
  permission TEXT,

  PRIMARY KEY (user_id, journal_id)
);
