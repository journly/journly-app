CREATE TABLE documents (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  trip_id UUID NOT NULL,
  filename TEXT NOT NULL,
  document_url TEXT,
  file_hash TEXT,
  file_type TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

  FOREIGN KEY(trip_id)
    REFERENCES trips(id)
    ON DELETE CASCADE
);
