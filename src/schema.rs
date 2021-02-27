table! {
    courses (id) {
        id -> Int4,
        title -> Varchar,
    }
}

table! {
    participations (user_id, course_id) {
        user_id -> Int4,
        course_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        gender -> Int4,
    }
}

joinable!(participations -> courses (course_id));
joinable!(participations -> users (user_id));

allow_tables_to_appear_in_same_query!(
    courses,
    participations,
    users,
);
