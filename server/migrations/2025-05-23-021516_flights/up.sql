CREATE TABLE flights (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  trip_id UUID NOT NULL,
  flight_code TEXT,
  departure_datetime TIMESTAMPTZ,
  arrival_datetime TIMESTAMPTZ,
  departure_location UUID,
  arrival_location UUID,
  from_document UUID,

  FOREIGN KEY (trip_id)
    REFERENCES trips(id)
    ON DELETE CASCADE,
  FOREIGN KEY (departure_location)
    REFERENCES locations(id),
  FOREIGN KEY (arrival_location)
    REFERENCES locations(id),
  FOREIGN KEY (from_document)
    REFERENCES documents(id)
);
