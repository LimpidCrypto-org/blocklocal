pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_users_table;
mod m20250314_213054_create_roles_table;
mod m20250314_213117_create_permissions_table;
mod m20250314_213216_create_user_has_role_table;
mod m20250314_213232_create_role_has_permission_table;
mod m20250314_220417_create_profiles_table;
mod m20250314_220502_create_profile_has_user_table;
mod m20250315_133755_create_networks_table;
mod m20250315_133828_create_nodes_table;
mod m20250315_135521_create_node_ports_table;
mod m20250315_161007_create_network_has_node_table;
mod m20250315_163951_seed_blocklocal_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_users_table::Migration),
            Box::new(m20250314_213054_create_roles_table::Migration),
            Box::new(m20250314_213117_create_permissions_table::Migration),
            Box::new(m20250314_213216_create_user_has_role_table::Migration),
            Box::new(m20250314_213232_create_role_has_permission_table::Migration),
            Box::new(m20250314_220417_create_profiles_table::Migration),
            Box::new(m20250314_220502_create_profile_has_user_table::Migration),
            Box::new(m20250315_133755_create_networks_table::Migration),
            Box::new(m20250315_133828_create_nodes_table::Migration),
            Box::new(m20250315_135521_create_node_ports_table::Migration),
            Box::new(m20250315_161007_create_network_has_node_table::Migration),
            Box::new(m20250315_163951_seed_blocklocal_user::Migration),
        ]
    }
}
