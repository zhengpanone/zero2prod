-- Add up migration script here
create table users(
    id integer primary key not null,
    openid text not null,
    session_key text not null,
    create_at TIMESTAMP not null DEFAULT CURRENT_TIMESTAMP,
    update_at TIMESTAMP not null DEFAULT CURRENT_TIMESTAMP
);

create unique index users_openid_index on users(openid);