CREATE TABLE expenses (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  budget_planner_id UUID NOT NULL,
  title TEXT,
  cost NUMERIC(10, 2),
  currency TEXT,

  FOREIGN KEY (budget_planner_id)
    REFERENCES budget_planners(id)
    ON DELETE CASCADE
);
