CREATE TABLE maps (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  map_type TEXT,
  title TEXT,
  trip_id UUID REFERENCES trips(id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
