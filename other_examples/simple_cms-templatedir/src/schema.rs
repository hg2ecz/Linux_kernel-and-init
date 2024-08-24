// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        #[max_length = 255]
        title -> Varchar,
        content -> Text,
        created_at -> Nullable<Timestamp>,
    }
}
