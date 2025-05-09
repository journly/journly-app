DROP TABLE IF EXISTS trips;
DROP TABLE IF EXISTS user_trips;

CREATE TABLE trips (
  id UUID PRIMARY KEY,
  owner_id UUID NOT NULL REFERENCES users,
  title TEXT NOT NULL DEFAULT 'Untitled trip',
  image_url TEXT,
  dates_id UUID NOT NULL REFERENCES dates ON DELETE CASCADE
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


DROP TRIGGER IF EXISTS on_trip_deletion ON trips;

CREATE OR REPLACE FUNCTION delete_date()
  RETURNS TRIGGER
  LANGUAGE PLPGSQL
  AS 
$$
BEGIN
  DELETE FROM dates
  WHERE dates.id = OLD.dates_id;

  RETURN OLD;
END;
$$;

CREATE TRIGGER on_trip_deletion
  AFTER DELETE 
  ON trips
  FOR EACH ROW
  EXECUTE PROCEDURE delete_date();


-- test records
INSERT INTO dates(id, start_date, end_date)
VALUES ('c4dbdb37-08e7-4ca8-bab1-3a339af004f9', '2025-12-12', '2025-12-21');

INSERT INTO trips(id, owner_id, title, dates_id)
VALUES ('c8381024-3f79-4a10-b5fe-06dc24e74bdc', '612e21ed-869b-4130-bb72-fc7549f93609', 'Example Trip','c4dbdb37-08e7-4ca8-bab1-3a339af004f9');
