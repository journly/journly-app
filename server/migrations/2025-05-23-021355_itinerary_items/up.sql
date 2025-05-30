CREATE TABLE itinerary_items (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  trip_id UUID NOT NULL,
  title TEXT NOT NULL,
  activity_type TEXT,
  location_id UUID,
  start_time TIMESTAMPTZ,
  end_time TIMESTAMPTZ,
  expense_id UUID,
  notes TEXT,

  FOREIGN KEY(trip_id)
    REFERENCES trips(id)
    ON DELETE CASCADE,
  FOREIGN KEY(location_id)
    REFERENCES locations(id),
  FOREIGN KEY(expense_id)
    REFERENCES expenses(id)
);
