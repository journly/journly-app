CREATE TABLE expenses (
  id UUID PRIMARY KEY,
  budgeting_tracker_id UUID NOT NULL REFERENCES budgeting_trackers,
  title TEXT NOT NULL,
  cost NUMERIC(10, 2) NOT NULL,
  expense_type TEXT NOT NULL,
  split_type TEXT NOT NULL
);

ALTER TABLE itinerary_activities ADD CONSTRAINT fk_ia_e FOREIGN KEY (expense_id) REFERENCES expenses(id);

CREATE TABLE expense_payers (
  expense_id UUID NOT NULL ON DELETE CASCADE,
  user_id UUID NOT NULL,
  PRIMARY KEY (expense_id, user_id)
);
