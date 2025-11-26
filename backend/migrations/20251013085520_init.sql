-- Add migration script here
do $$
BEGIN
IF NOT EXISTS (SELECT 1 FROM pg_type where typname = 'user_status_enum') THEN
	CREATE TYPE user_status_enum as ENUM ('active', 'inactive','banned');
END IF;

end$$;


-- Create sys_user table
DROP TABLE IF EXISTS sys_user CASCADE;

CREATE TABLE IF NOT EXISTS sys_user (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    username VARCHAR(100) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    status user_status_enum NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_id VARCHAR(255) not null DEFAULT '1',
    created_by VARCHAR(255) NOT NULL DEFAULT 'system',
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_id VARCHAR(255) not null DEFAULT '1',
    updated_by VARCHAR(255) NOT NULL DEFAULT 'system',
    is_deleted boolean not null default false,
    deleted_at TIMESTAMP
);

comment on TABLE sys_user is '用户表';
comment on COLUMN sys_user.email is '邮箱';
comment on COLUMN sys_user.email_verified is '邮箱是否校验';
comment on COLUMN sys_user.username is '用户名';
comment on COLUMN sys_user.password_hash is '密码';
comment on COLUMN sys_user.status is '用户状态';
comment on COLUMN sys_user.created_at is '创建时间';
comment on COLUMN sys_user.created_id is '创建人ID';
comment on COLUMN sys_user.created_by is '创建人';
comment on COLUMN sys_user.updated_at is '更新时间';
comment on COLUMN sys_user.updated_id is '更新人ID';
comment on COLUMN sys_user.updated_by is '更新人';

-- Create index on email for faster lookups
CREATE INDEX idx_users_email ON sys_user(email);

-- Create updated_at trigger function
CREATE OR REPLACE FUNCTION trg_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at := NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

DROP TRIGGER IF EXISTS set_sys_user_updated_at ON sys_user;

-- Create trigger for sys_user table
CREATE TRIGGER set_sys_user_updated_at
    BEFORE UPDATE ON sys_user
    FOR EACH ROW
    EXECUTE FUNCTION trg_set_timestamp();


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
