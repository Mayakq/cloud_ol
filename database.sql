CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
create table if not exists "users"
(
    id       uuid not null ,
    login    varchar(128)  not null,
    password varchar(1024) not null,
    jswt     varchar(2048)
)