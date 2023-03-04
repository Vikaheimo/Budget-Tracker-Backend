// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}
