// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        pfp -> Varchar,
    }
}

diesel::table! {
    courses (id) {
        id -> Int4,
        slug -> Varchar,
        name -> Varchar,
        description -> Varchar,
        image -> Varchar,
    }
}

diesel::table! {
    lessons (id) {
        id -> Int4,
        reference -> Varchar,
        slug -> Varchar,
        name -> Varchar,
        content -> Varchar,
        image -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    courses,
    lessons,
);
