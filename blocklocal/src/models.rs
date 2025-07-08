use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::blockchains)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Node {
    pub id: i32,
    pub hash: String,
    pub config_url: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::blockchains)]
pub struct NewNode<'a> {
    pub hash: &'a str,
    pub config_url: &'a str,
}
