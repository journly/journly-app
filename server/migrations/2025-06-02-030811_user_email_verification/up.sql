ALTER TABLE users
ADD COLUMN is_verified BOOLEAN DEFAULT FALSE,
ADD COLUMN email_verification_token UUID,
ADD COLUMN token_expires_at TIMESTAMPTZ;
