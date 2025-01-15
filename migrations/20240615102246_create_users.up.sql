-- Add up migration script here
create table users(
    id integer primary key not null,
    openid text  ,
    username text not null,
    `password` text not null,
    email text not null,
    session_key text ,
    create_at TIMESTAMP not null DEFAULT CURRENT_TIMESTAMP,
    update_at TIMESTAMP not null DEFAULT CURRENT_TIMESTAMP,
    locked_at TIMESTAMP
);

create unique index users_openid_index on users(openid);