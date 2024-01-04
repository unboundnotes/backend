// @generated automatically by Diesel CLI.

diesel::table! {
    atoms (slot_uuid, idx) {
        slot_uuid -> Uuid,
        idx -> Int4,
        typ -> Varchar,
        data -> Nullable<Text>,
    }
}

diesel::table! {
    data_source (key) {
        key -> Text,
        value -> Text,
    }
}

diesel::table! {
    pages (uuid) {
        workspace_uuid -> Uuid,
        uuid -> Uuid,
        title -> Varchar,
        image -> Nullable<Varchar>,
    }
}

diesel::table! {
    slots (uuid) {
        page_uuid -> Uuid,
        uuid -> Uuid,
        order -> Bpchar,
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

diesel::table! {
    workspaces (uuid) {
        uuid -> Uuid,
        name -> Varchar,
        image -> Varchar,
    }
}

diesel::joinable!(atoms -> slots (slot_uuid));
diesel::joinable!(pages -> workspaces (workspace_uuid));
diesel::joinable!(slots -> pages (page_uuid));

diesel::allow_tables_to_appear_in_same_query!(
    atoms,
    data_source,
    pages,
    slots,
    users,
    workspaces,
);
