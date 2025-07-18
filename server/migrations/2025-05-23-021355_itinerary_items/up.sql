CREATE TABLE itinerary_items (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  trip_id UUID NOT NULL REFERENCES trips(id) ON DELETE CASCADE,
  title TEXT,
  activity_type TEXT,
  location_id UUID REFERENCES locations(id),
  start_time TIMESTAMPTZ,
  end_time TIMESTAMPTZ,
  expense_id UUID REFERENCES expenses(id),
  notes TEXT
);
