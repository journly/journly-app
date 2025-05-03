DROP TABLE IF EXISTS trips;
DROP TABLE IF EXISTS user_trips;

CREATE TABLE trips (
  id UUID PRIMARY KEY,
  owner_id UUID NOT NULL REFERENCES users,
  title TEXT NOT NULL DEFAULT 'Untitled trip',
  image_url TEXT,
  dates_id UUID REFERENCES dates NOT NULL
);

CREATE TABLE user_trips (
  trip_id UUID NOT NULL REFERENCES trips ON DELETE CASCADE,
  user_id UUID NOT NULL REFERENCES users ON DELETE CASCADE,
  PRIMARY KEY (trip_id, user_id)
);

DROP VIEW IF EXISTS trip_details;

CREATE OR REPLACE VIEW trip_details AS
SELECT
	trips.id as id,
  owner_id,
	title,
  image_url,
	start_date,
	end_date
FROM 
	trips INNER JOIN dates ON trips.dates_id = dates.id;
