CREATE TABLE replicache_space (
  id TEXT PRIMARY KEY,
  version INTEGER NOT NULL
);

CREATE TABLE replicache_client_group (
  id TEXT PRIMARY KEY,
  user_id UUID NOT NULL,
  space_id TEXT NOT NULL
);

CREATE TABLE replicache_client (
  id TEXT PRIMARY KEY,
  client_group_id TEXT NOT NULL REFERENCES replicache_client_group(id) ON DELETE CASCADE,
  last_mutation_id INTEGER NOT NULL,
  last_modified_version INTEGER NOT NULL
);

