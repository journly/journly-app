CREATE TABLE budget_planners (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  trip_id UUID NOT NULL REFERENCES trips(id) ON DELETE CASCADE,
  total_budget NUMERIC(10,2),
  currency TEXT,
  accommodation_budget NUMERIC(10,2),
  transportation_budget NUMERIC(10,2),
  food_dining_budget NUMERIC(10,2),
  activities_budget NUMERIC(10,2),
  shopping_budget NUMERIC(10,2)
);
