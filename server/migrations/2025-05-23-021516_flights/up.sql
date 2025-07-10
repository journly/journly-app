CREATE TABLE flights (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  trip_id UUID NOT NULL REFERENCES trips(id) ON DELETE CASCADE,
  flight_code TEXT,
  departure_datetime TIMESTAMPTZ,
  arrival_datetime TIMESTAMPTZ,
  departure_location UUID REFERENCES locations(id),
  arrival_location UUID REFERENCES locations(id),
  from_document UUID REFERENCES documents(id)
);
