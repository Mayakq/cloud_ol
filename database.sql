CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
drop table if EXISTS "users";
drop table if EXISTS  "photo";
drop table if EXISTS "albums";
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
    name varchar(256) not null,
    public bool default false,
    path varchar(8192) not null ,
    owner uuid not null,
);
create table if not exists "photo"
(
    id   uuid not null default uuid_generate_v4(),
    public bool default false,
    path varchar(8192) not null ,
    owner uuid not null,
    extension varchar(256) not null,
    album uuid

)

