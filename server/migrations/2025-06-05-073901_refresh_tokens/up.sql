CREATE TABLE refresh_tokens (
  token TEXT PRIMARY KEY,
  user_id UUID REFERENCES users(id) ON DELETE CASCADE,
  expires_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  parent_token TEXT REFERENCES refresh_tokens(token) ON DELETE CASCADE,
  revoked BOOLEAN NOT NULL DEFAULT FALSE
);
