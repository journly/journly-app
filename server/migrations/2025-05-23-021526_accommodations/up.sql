CREATE TABLE accommodations (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  trip_id UUID NOT NULL REFERENCES trips(id) ON DELETE CASCADE,
  check_in_datetime TIMESTAMPTZ,
  check_out_datetime TIMESTAMPTZ,
  location UUID REFERENCES locations(id),
  from_document UUID REFERENCES documents(id)
;
