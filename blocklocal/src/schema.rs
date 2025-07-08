// @generated automatically by Diesel CLI.

diesel::table! {
    blockchains (id) {
        id -> Int4,
        #[max_length = 64]
        hash -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        config_url -> Text,
    }
}
