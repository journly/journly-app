DROP TABLE IF EXISTS itineraries;

CREATE TABLE itineraries (
  id UUID PRIMARY KEY,
  widget_id UUID NOT NULL REFERENCES widgets,
  dates_id UUID NOT NULL REFERENCES dates,
  map_id UUID NOT NULL REFERENCES maps
);
