// @generated automatically by Diesel CLI.

diesel::table! {
    games (id) {
        id -> Text,
        name -> Text,
        creator -> Text,
        plays -> Integer,
        created_at -> Nullable<Timestamp>,
    }
}
