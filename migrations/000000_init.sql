create extension pgcrypto;

create table if not exists credentials (
    id bigserial primary key,
    username varchar(50) not null unique,
    password text not null
);