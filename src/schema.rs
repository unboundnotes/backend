// @generated automatically by Diesel CLI.

diesel::table! {
    data_source (key) {
        key -> Text,
        value -> Text,
    }
}

diesel::table! {
    users (uuid) {
        uuid -> Uuid,
        email -> Varchar,
        username -> Varchar,
        password -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    data_source,
    users,
);
