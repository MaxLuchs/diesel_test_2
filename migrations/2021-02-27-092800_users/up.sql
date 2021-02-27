create table users
(
    -- Only integer types can be auto increment
    id serial primary key,
    name varchar not null,
    gender int not null
);


