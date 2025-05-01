CREATE TABLE markers (
  id UUID PRIMARY KEY,
  coordinates_id UUID NOT NULL REFERENCES coordinates,
  itinerary_activity_id UUID NOT NULL REFERENCES itinerary_activities
);
