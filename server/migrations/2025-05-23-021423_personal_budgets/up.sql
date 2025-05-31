CREATE TABLE personal_budgets (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  trip_id UUID NOT NULL,
  user_id UUID NOT NULL,
  total_budget NUMERIC(10,2),
  accommodation_budget NUMERIC(10,2),
  transportation_budget NUMERIC(10,2),
  food_dining_budget NUMERIC(10,2),
  activities_budget NUMERIC(10,2),
  shopping_budget NUMERIC(10,2),
  currency TEXT,
  personal_budget_enabled BOOLEAN NOT NULL DEFAULT FALSE,

  FOREIGN KEY (trip_id)
    REFERENCES trips(id)
    ON DELETE CASCADE,
  FOREIGN KEY (user_id)
    REFERENCES users(id)
    ON DELETE CASCADE
);
