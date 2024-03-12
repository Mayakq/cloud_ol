CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
create table if not exists "users"
(
    id       uuid not null default uuid_generate_v4(),
    login    varchar(128)  not null,
    password varchar(1024) not null,
    jswt     varchar(2048)
)