use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20220101_000001_create_users_table::Users, m20250314_213054_create_roles_table::Roles,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

const PK_USER_HAS_ROLE_USER_ID_PERMISSION_ID: &str = "PK_userHasRole_userId_permissionId";
const FK_USER_HAS_ROLE_USER_ID: &str = "FK_userHasRole_userId";
const FK_USER_HAS_ROLE_ROLE_ID: &str = "FK_userHasRole_roleId";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserHasRole::Table)
                    .if_not_exists()
                    .col(integer(UserHasRole::UserId))
                    .col(integer(UserHasRole::RoleId))
                    .primary_key(
                        Index::create()
                            .col(UserHasRole::UserId)
                            .col(UserHasRole::RoleId)
                            .name(PK_USER_HAS_ROLE_USER_ID_PERMISSION_ID),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(UserHasRole::Table, UserHasRole::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .name(FK_USER_HAS_ROLE_USER_ID)
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(UserHasRole::Table, UserHasRole::RoleId)
                    .to(Roles::Table, Roles::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .name(FK_USER_HAS_ROLE_ROLE_ID)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(UserHasRole::Table)
                    .name(FK_USER_HAS_ROLE_USER_ID)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(UserHasRole::Table)
                    .name(FK_USER_HAS_ROLE_ROLE_ID)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(UserHasRole::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserHasRole {
    Table,
    UserId,
    RoleId,
}
