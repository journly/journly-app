ALTER TABLE refresh_tokens
ADD COLUMN parent_token TEXT,
ADD CONSTRAINT fk_parent_refresh_token FOREIGN KEY (parent_token) REFERENCES refresh_tokens(token),
ADD COLUMN revoked BOOLEAN DEFAULT FALSE;
