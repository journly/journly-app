CREATE TABLE trips (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  owner_id UUID NOT NULL,
  title TEXT,
  banner_image TEXT,
  start_date DATE,
  end_date DATE,
  no_collaborators INTEGER NOT NULL DEFAULT 1,

  FOREIGN KEY(owner_id) REFERENCES users(id)
);
