#![allow(dead_code)]

use entity::{profiles, users};
use sea_orm::ActiveValue::Set;
use sea_orm::{entity::*, QueryFilter};

use super::DbResult;
use super::DB;

/// Extension trait for the Model of the Profile entity.
/// This trait provides methods to query the profile entity.
pub trait ProfileModelExt {
    /// Create a new profile with the given name.
    async fn create<'a>(name: &'a str) -> DbResult<profiles::Model>;
    /// Find a profile by its ID.
    async fn find_by_id(id: i32) -> DbResult<Option<profiles::Model>>;
    /// Find a profile by its name.
    async fn find_by_name(name: &str) -> DbResult<Option<profiles::Model>>;
    /// Find all profiles.
    async fn find_all() -> DbResult<Vec<profiles::Model>>;
    /// Find all profiles for a given user.
    async fn find_all_user_profiles(user_id: i32) -> DbResult<Vec<profiles::Model>>;
}

/// Extension trait for the ActiveModel of the Profile entity.
/// This trait provides method to modify the profile.
pub trait ProfileActiveModelExt {
    /// Set the name of the profile.
    async fn set_name<'a>(self, name: &'a str) -> DbResult<profiles::Model>;
}

impl ProfileModelExt for profiles::Model {
    async fn create<'a>(name: &'a str) -> DbResult<profiles::Model> {
        let db = &*DB;
        let profile = profiles::ActiveModel {
            name: Set(name.to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(profile)
    }

    async fn find_by_id(id: i32) -> DbResult<Option<profiles::Model>> {
        let db = &*DB;
        let profile = profiles::Entity::find_by_id(id).one(db).await?;

        Ok(profile)
    }

    async fn find_by_name(name: &str) -> DbResult<Option<profiles::Model>> {
        let db = &*DB;
        let profile = profiles::Entity::find()
            .filter(profiles::Column::Name.contains(name))
            .one(db)
            .await?;

        Ok(profile)
    }

    async fn find_all() -> DbResult<Vec<profiles::Model>> {
        let db = &*DB;
        let profiles = profiles::Entity::find().all(db).await?;

        Ok(profiles)
    }

    async fn find_all_user_profiles(user_id: i32) -> DbResult<Vec<profiles::Model>> {
        let db = &*DB;
        let profiles = profiles::Entity::find()
            .inner_join(users::Entity)
            .filter(users::Column::Id.eq(user_id))
            .all(db)
            .await?;

        Ok(profiles)
    }
}

impl ProfileActiveModelExt for profiles::ActiveModel {
    async fn set_name<'a>(mut self, name: &'a str) -> DbResult<profiles::Model> {
        self.name = Set(name.to_owned());
        let db = &*DB;

        Ok(self.update(db).await?)
    }
}
