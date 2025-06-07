ALTER TABLE refresh_tokens
DROP CONSTRAINT fk_parent_refresh_token,
DROP COLUMN IF EXISTS parent_token,
DROP COLUMN IF EXISTS revoked;
