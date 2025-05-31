CREATE TABLE accommodations (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  trip_id UUID NOT NULL,
  check_in_datetime TIMESTAMPTZ,
  check_out_datetime TIMESTAMPTZ,
  location UUID,
  from_document UUID,

  FOREIGN KEY (trip_id)
    REFERENCES trips(id)
    ON DELETE CASCADE,
  FOREIGN KEY (location)
    REFERENCES locations(id),
  FOREIGN KEY (from_document)
    REFERENCES documents(id)
);
