CREATE TABLE user_verification_codes(
  email TEXT PRIMARY KEY,
  verification_code INTEGER NOT NULL,
  expires_at TIMESTAMPTZ NOT NULL
);
