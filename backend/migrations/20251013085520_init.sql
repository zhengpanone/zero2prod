-- Add migration script here
do $$
BEGIN
IF NOT EXISTS (SELECT 1 FROM pg_type where typname = 'user_status_enum') THEN
	CREATE TYPE user_status_enum as ENUM ('active', 'inactive','banned');
END IF;

end$$;


-- Create sys_user table
DROP TABLE IF EXISTS sys_user;

CREATE TABLE IF NOT EXISTS sys_user (
    id VARCHAR(36) PRIMARY KEY DEFAULT gen_random_uuid(),
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


DROP TABLE IF EXISTS sys_user_role;
CREATE TABLE IF NOT EXISTS sys_user_role (
    id VARCHAR(36) PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id VARCHAR(36) NOT NULL ,
    role_id VARCHAR(36) NOT NULL ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_id VARCHAR(255) not null DEFAULT '1',
    created_by VARCHAR(255) NOT NULL DEFAULT 'system',
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_id VARCHAR(255) not null DEFAULT '1',
    updated_by VARCHAR(255) NOT NULL DEFAULT 'system',
    is_deleted boolean not null default false,
    deleted_at TIMESTAMP
);

comment on TABLE sys_user_role is '用户角色表';
comment on COLUMN sys_user_role.user_id is '用户ID';
comment on COLUMN sys_user_role.role_id is '角色ID';


do $$
BEGIN
IF NOT EXISTS (SELECT 1 FROM pg_type where typname = 'role_status_enum') THEN
	CREATE TYPE role_status_enum as ENUM ('active', 'inactive','banned');
END IF;

end$$;

DROP TABLE IF EXISTS sys_role;
CREATE TABLE IF NOT EXISTS sys_role (
    id VARCHAR(36) PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    code VARCHAR(255) NOT NULL UNIQUE,
    status role_status_enum NOT NULL DEFAULT 'active',
    order_num int not null default 1,
    remark VARCHAR(255),
    description VARCHAR(255),
    is_default boolean not null default false,
    is_protected boolean not null default false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_id VARCHAR(255) not null DEFAULT '1',
    created_by VARCHAR(255) NOT NULL DEFAULT 'system',
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_id VARCHAR(255) not null DEFAULT '1',
    updated_by VARCHAR(255) NOT NULL DEFAULT 'system',
    is_deleted boolean not null default false,
    deleted_at TIMESTAMP
);

comment on TABLE sys_role is '角色表';
comment on COLUMN sys_role.name is '角色名称';
comment on COLUMN sys_role.description is '角色描述';
comment on COLUMN sys_role.status is '角色状态';
comment on COLUMN sys_role.is_default is '是否默认角色';
comment on COLUMN sys_role.is_protected is '是否保护角色';
comment on COLUMN sys_role.is_deleted is '是否删除';



-- Create refresh_tokens table
DROP TABLE IF EXISTS refresh_tokens;
CREATE TABLE IF NOT EXISTS refresh_tokens (
jti VARCHAR(36) PRIMARY KEY,
user_id VARCHAR(36) NOT NULL ,
expires_at TIMESTAMPTZ NOT NULL,
revoked BOOLEAN NOT NULL DEFAULT FALSE,
created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_refresh_user ON refresh_tokens(user_id);

-- Create email_verifications table
DROP TABLE IF EXISTS email_verifications;
CREATE TABLE IF NOT EXISTS email_verifications (
user_id VARCHAR(36) PRIMARY KEY ,
token VARCHAR(36) NOT NULL,
expires_at TIMESTAMPTZ NOT NULL,
created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Create password_resets table
DROP TABLE IF EXISTS password_resets;
CREATE TABLE IF NOT EXISTS password_resets (
user_id VARCHAR(36) PRIMARY KEY ,
token VARCHAR(36) NOT NULL,
expires_at TIMESTAMPTZ NOT NULL,
created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Create sys_dict_type table
DROP TABLE IF EXISTS sys_dict_type;
CREATE TABLE IF NOT EXISTS sys_dict_type (
    id VARCHAR(36) PRIMARY KEY ,
    dict_type VARCHAR(36) NOT NULL,
    order_num int NOT NULL DEFAULT 1,
    description VARCHAR(255),
    system_flag boolean not null default false,
    status VARCHAR(10) NOT NULL default '1',
    remark VARCHAR(10),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_id VARCHAR(255) not null DEFAULT '1',
    created_by VARCHAR(255) NOT NULL DEFAULT 'system',
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_id VARCHAR(255) not null DEFAULT '1',
    updated_by VARCHAR(255) NOT NULL DEFAULT 'system',
    is_deleted boolean not null default false,
    deleted_at TIMESTAMP
);
