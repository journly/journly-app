CREATE TABLE refresh_tokens (
  token TEXT PRIMARY KEY,
  user_id UUID REFERENCES users(id),
  expires_at TIMESTAMP,
  created_at TIMESTAMP DEFAULT NOW()
)
