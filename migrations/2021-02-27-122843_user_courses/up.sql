create table participations
(
    user_id int not null
        constraint participations_users_id_fk
            references users,
    course_id int not null
        constraint participations_courses_id_fk
            references courses,
    constraint participations_pk
        primary key (user_id, course_id)
);