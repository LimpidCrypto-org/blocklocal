diesel::table! {
    nodes (id) {
        id -> Int4,
        #[max_length = 64]
        hash -> Varchar,
        url -> Text,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}