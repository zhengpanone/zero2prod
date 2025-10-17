-- Add migration script here
-- Create sys_user table
DROP TABLE IF EXISTS sys_user;
CREATE TABLE IF NOT EXISTS sys_user (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    username VARCHAR(100) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index on email for faster lookups
CREATE INDEX idx_users_email ON sys_user(email);

-- Create updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create trigger for sys_user table
CREATE TRIGGER update_sys_user_updated_at
    BEFORE UPDATE ON sys_user
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();


-- Create refresh_tokens table
DROP TABLE IF EXISTS refresh_tokens;
CREATE TABLE IF NOT EXISTS refresh_tokens (
jti UUID PRIMARY KEY,
user_id UUID NOT NULL REFERENCES sys_user(id) ON DELETE CASCADE,
expires_at TIMESTAMPTZ NOT NULL,
revoked BOOLEAN NOT NULL DEFAULT FALSE,
created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_refresh_user ON refresh_tokens(user_id);

-- Create email_verifications table
DROP TABLE IF EXISTS email_verifications;
CREATE TABLE IF NOT EXISTS email_verifications (
user_id UUID PRIMARY KEY REFERENCES sys_user(id) ON DELETE CASCADE,
token UUID NOT NULL,
expires_at TIMESTAMPTZ NOT NULL,
created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Create password_resets table
DROP TABLE IF EXISTS password_resets;
CREATE TABLE IF NOT EXISTS password_resets (
user_id UUID PRIMARY KEY REFERENCES sys_user(id) ON DELETE CASCADE,
token UUID NOT NULL,
expires_at TIMESTAMPTZ NOT NULL,
created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
