use entity::{profile_has_user, profiles, roles, users};
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{entity::*, query::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

const BLOCKLOCAL_USER_ID: i32 = 1;
const BLOCKLOCAL_USER_NAME: &str = "blocklocal";
const ADMIN_ROLE_ID: i32 = 1;
const ADMIN_ROLE_NAME: &str = "admin";
const BLOCKLOCAL_PROFILE_ID: i32 = 1;
const BLOCKLOCAL_PROFILE_NAME: &str = "blocklocal";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let tx = db.begin().await?;

        users::ActiveModel {
            id: Set(BLOCKLOCAL_USER_ID),
            name: Set(BLOCKLOCAL_USER_NAME.to_owned()),
            ..Default::default()
        }
        .insert(&tx)
        .await?;
        roles::ActiveModel {
            id: Set(ADMIN_ROLE_ID),
            name: Set(ADMIN_ROLE_NAME.to_owned()),
            ..Default::default()
        }
        .insert(&tx)
        .await?;
        profiles::ActiveModel {
            id: Set(BLOCKLOCAL_PROFILE_ID),
            name: Set(BLOCKLOCAL_PROFILE_NAME.to_owned()),
            ..Default::default()
        }
        .insert(&tx)
        .await?;
        profile_has_user::ActiveModel {
            profile_id: Set(BLOCKLOCAL_PROFILE_ID),
            user_id: Set(BLOCKLOCAL_USER_ID),
            ..Default::default()
        }
        .insert(&tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let tx = db.begin().await?;

        users::ActiveModel {
            id: Set(BLOCKLOCAL_USER_ID),
            ..Default::default()
        }
        .delete(&tx)
        .await?;
        roles::ActiveModel {
            id: Set(ADMIN_ROLE_ID),
            ..Default::default()
        }
        .delete(&tx)
        .await?;
        profiles::ActiveModel {
            id: Set(BLOCKLOCAL_PROFILE_ID),
            ..Default::default()
        }
        .delete(&tx)
        .await?;
        profile_has_user::ActiveModel {
            profile_id: Set(BLOCKLOCAL_PROFILE_ID),
            user_id: Set(BLOCKLOCAL_USER_ID),
            ..Default::default()
        }
        .delete(&tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }
}
