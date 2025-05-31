CREATE TABLE expenses (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  trip_id UUID NOT NULL,
  title TEXT,
  cost NUMERIC(10, 2),
  currency TEXT,

  FOREIGN KEY (trip_id)
    REFERENCES trips(id)
    ON DELETE CASCADE
);
