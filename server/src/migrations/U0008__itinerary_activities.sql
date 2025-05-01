CREATE TABLE itinerary_activities (
  id UUID PRIMARY KEY,
  itinerary_id UUID NOT NULL REFERENCES itineraries,
  date_id UUID NOT NULL REFERENCES dates,
  start_time DATETIME,
  end_time DATETIME,
  notes TEXT,
);
