// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        user_id -> Integer,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}
