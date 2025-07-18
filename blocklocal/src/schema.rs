diesel::table! {
    blockchain_groups (id) {
        id -> Integer,
        public_id -> Uuid,

        // The group name is a human-readable identifier for the blockchain group.
        name -> Text,
        // The group description provides additional context about the blockchain group.
        description -> Nullable<Text>,

        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    blockchain_configs (id) {
        id -> Integer,
        public_id -> Uuid,

        // The configs origin can be a URL or a local file path.
        config_origin -> Text,
        blockchain_group_id -> Integer,
        data -> Bytea,

        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(blockchain_configs -> blockchain_groups (blockchain_group_id));
diesel::allow_tables_to_appear_in_same_query!(
    blockchain_groups,
    blockchain_configs,
);