create table courses
(
    id serial
        constraint courses_pk
            primary key,
    title varchar not null
);

-- Your SQL goes here