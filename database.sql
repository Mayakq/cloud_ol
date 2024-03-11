CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
create table if not exists "user"
(
    id       uuid default uuid_generate_v4(),
    name     varchar(128)  not null,
    login     varchar(128)  not null,
    email     varchar(128)  not null,
    password varchar(1024) not null
)