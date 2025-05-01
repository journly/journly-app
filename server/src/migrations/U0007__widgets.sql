CREATE TABLE widgets (
  id UUID PRIMARY KEY,
  section_id UUID NOT NULL REFERENCES sections,
  widget_type TEXT NOT NULL,
  section_order_rank SMALLINT NOT NULL,
  width SMALLINT NOT NULL,
  height SMALLINT NOT NULL,
  content jsonb NOT NULL
);
