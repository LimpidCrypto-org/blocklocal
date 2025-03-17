#![allow(dead_code)]

use entity::networks;
use ipnet::IpSubnets;
use sea_orm::ActiveValue::Set;
use sea_orm::{entity::*, QueryFilter};

use super::{DbResult, DB};

pub trait NetworkActiveModelExt {
    /// Set the name of the network.
    async fn set_name<'a>(self, name: &'a str) -> DbResult<networks::Model>;
}

/// Extension trait for the Model of the Network entity.
/// This trait provides methods to query the network entity.
pub trait NetworkModelExt {
    /// Create a new network with the given name, subnet, and bridge.
    async fn create<'a>(
        name: &'a str,
        subnet: &'a str,
        bridge: Option<&'a str>,
    ) -> DbResult<networks::Model>;
    /// Find a network by its ID.
    async fn find_by_id(id: i32) -> DbResult<Option<networks::Model>>;
    /// Find a network by its name.
    async fn find_by_name(name: &str) -> DbResult<Option<networks::Model>>;
}

impl NetworkModelExt for networks::Model {
    async fn create<'a>(
        name: &'a str,
        subnet: &'a str,
        bridge: Option<&'a str>,
    ) -> DbResult<networks::Model> {
        let db = &*DB;
        let network = networks::ActiveModel {
            name: Set(name.to_owned()),
            subnet: Set(subnet.to_owned()),
            driver: Set(bridge.unwrap_or("bridge").to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(network)
    }

    async fn find_by_id(id: i32) -> DbResult<Option<networks::Model>> {
        let db = &*DB;
        let network = networks::Entity::find_by_id(id).one(db).await?;

        Ok(network)
    }

    async fn find_by_name(name: &str) -> DbResult<Option<networks::Model>> {
        let db = &*DB;
        let network = networks::Entity::find()
            .filter(networks::Column::Name.contains(name))
            .one(db)
            .await?;

        Ok(network)
    }
}
