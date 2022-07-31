-- Add migration script here
create table users (
    name varchar (50) unique not null,
    id serial primary key
);