// @generated automatically by Diesel CLI.

diesel::table! {
    _prisma_migrations (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 64]
        checksum -> Varchar,
        finished_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        migration_name -> Varchar,
        logs -> Nullable<Text>,
        rolled_back_at -> Nullable<Timestamptz>,
        started_at -> Timestamptz,
        applied_steps_count -> Int4,
    }
}

diesel::table! {
    comments (id) {
        id -> Text,
        id_user -> Text,
        id_post -> Text,
        content -> Text,
    }
}

diesel::table! {
    posts (id) {
        id -> Text,
        title -> Text,
        content -> Text,
        id_user -> Text,
        created_at -> Timestamp,
        height -> Int4,
        version -> Text,
        width -> Int4,
        url_bucket -> Text,
    }
}

diesel::table! {
    ratings (id) {
        id -> Text,
        value -> Int4,
        id_user -> Text,
        id_post -> Text,
        createdAt -> Timestamp,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    sessions (id) {
        id -> Text,
        id_user -> Text,
        refresh_token -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        name -> Text,
        email -> Text,
        password -> Text,
    }
}

diesel::joinable!(comments -> posts (id_post));
diesel::joinable!(comments -> users (id_user));
diesel::joinable!(posts -> users (id_user));
diesel::joinable!(ratings -> posts (id_post));
diesel::joinable!(ratings -> users (id_user));
diesel::joinable!(sessions -> users (id_user));

diesel::allow_tables_to_appear_in_same_query!(
    _prisma_migrations,
    comments,
    posts,
    ratings,
    sessions,
    users,
);
