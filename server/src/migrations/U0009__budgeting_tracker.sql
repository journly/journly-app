DROP TABLE IF EXISTS budgeting_trackers;

CREATE TABLE budgeting_trackers (
  id UUID PRIMARY KEY,
  widget_id UUID NOT NULL REFERENCES widgets,
  title TEXT NOT NULL,
  total_budget NUMERIC(10, 2),
  currency VARCHAR(3) NOT NULL
);
