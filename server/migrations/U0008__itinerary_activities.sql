DROP TABLE IF EXISTS itinerary_activities;

CREATE TABLE itinerary_activities (
  id UUID PRIMARY KEY,
  itinerary_id UUID NOT NULL REFERENCES itineraries,
  date_id UUID NOT NULL REFERENCES dates,
  start_time TIMESTAMP WITH TIME ZONE,
  end_time TIMESTAMP WITH TIME ZONE,
  notes TEXT
);
