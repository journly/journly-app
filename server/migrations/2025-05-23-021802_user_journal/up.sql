CREATE TABLE user_journal (
  user_id UUID NOT NULL,
  journal_id UUID NOT NULL,
  permission TEXT,

  PRIMARY KEY (user_id, journal_id),
  FOREIGN KEY (user_id)
    REFERENCES users(id),
  FOREIGN KEY (journal_id)
    REFERENCES journals(id)
    ON DELETE CASCADE
);
