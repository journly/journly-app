ALTER TABLE users
DROP COLUMN IF EXISTS is_verified,
DROP COLUMN IF EXISTS email_verification_token,
DROP COLUMN IF EXISTS token_expires_at;
