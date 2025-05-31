CREATE TABLE expense_payers (
  expense_id UUID NOT NULL,
  user_id UUID NOT NULL,

  PRIMARY KEY (expense_id, user_id),
  FOREIGN KEY (expense_id)
    REFERENCES expenses(id)
    ON DELETE CASCADE,
  FOREIGN KEY (user_id)
    REFERENCES users(id)
    ON DELETE CASCADE
);
