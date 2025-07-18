CREATE TABLE expenses (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  trip_id UUID NOT NULL REFERENCES trips(id) ON DELETE CASCADE,
  title TEXT,
  cost NUMERIC(10, 2) NOT NULL,
  currency TEXT NOT NULL
);
