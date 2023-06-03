// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        pfp -> Varchar,
    }
}
