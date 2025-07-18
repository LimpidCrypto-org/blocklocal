use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::blockchain_groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BlockchainGroup {
    pub id: i32,
    pub public_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::blockchain_configs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BlockchainConfig {
    pub id: i32,
    pub public_id: Uuid,
    pub config_origin: String,
    pub blockchain_group_id: i32,
    pub data: Vec<u8>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
