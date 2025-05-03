DROP TABLE IF EXISTS files;
DROP TABLE IF EXISTS attachments;

CREATE TABLE files (
  id UUID PRIMARY KEY,
  user_id UUID NOT NULL,
  file_name TEXT NOT NULL,
  file_url TEXT NOT NULL UNIQUE,
  file_hash TEXT NOT NULL UNIQUE,
  content_type TEXT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE attachments (
  trip_id UUID NOT NULL REFERENCES trips,
  file_id UUID NOT NULL REFERENCES files,
  activity_id UUID REFERENCES itinerary_activities,
  PRIMARY KEY (trip_id, file_id)
);
