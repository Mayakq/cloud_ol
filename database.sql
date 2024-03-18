CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
create table if not exists "users"
(
    id       uuid          not null default uuid_generate_v4(),
    login    varchar(128)  not null,
    password varchar(1024) not null,
    jswt     varchar(2048)
);
create table if not exists "albums"
(
    id   uuid not null default uuid_generate_v4(),
    path varchar(8192),
    owner uuid not null,
    joint_author uuid default null,
    users_with_access uuid default null
);
create table if not exists "photo"
(
    id uuid not null default uuid_generate_v4(),
    album uuid not null,
    title varchar(1024) not null

)