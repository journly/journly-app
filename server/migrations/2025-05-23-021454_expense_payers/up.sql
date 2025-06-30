CREATE TABLE expense_payers (
  expense_id UUID NOT NULL REFERENCES expenses(id) ON DELETE CASCADE,
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

  PRIMARY KEY (expense_id, user_id)
);
