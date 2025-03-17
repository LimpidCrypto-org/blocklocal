use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20250314_213054_create_roles_table::Roles,
    m20250314_213117_create_permissions_table::Permissions,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

const FK_ROLE_HAS_PERMISSION_ROLE_ID: &str = "FK_roleHasPermission_roleId";
const FK_ROLE_HAS_PERMISSION_PERMISSION_ID: &str = "FK_roleHasPermission_permissionId";
const PK_ROLE_HAS_PERMISSION_ROLE_ID_PERMISSION_ID: &str =
    "PK_roleHasPermission_roleId_permissionId";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RoleHasPermission::Table)
                    .if_not_exists()
                    .col(integer(RoleHasPermission::RoleId))
                    .col(integer(RoleHasPermission::PermissionId))
                    .primary_key(
                        Index::create()
                            .col(RoleHasPermission::RoleId)
                            .col(RoleHasPermission::PermissionId)
                            .name(PK_ROLE_HAS_PERMISSION_ROLE_ID_PERMISSION_ID),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(RoleHasPermission::Table, RoleHasPermission::RoleId)
                    .to(Roles::Table, Roles::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .name(FK_ROLE_HAS_PERMISSION_ROLE_ID)
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(RoleHasPermission::Table, RoleHasPermission::PermissionId)
                    .to(Permissions::Table, Permissions::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .name(FK_ROLE_HAS_PERMISSION_PERMISSION_ID)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(RoleHasPermission::Table)
                    .name(FK_ROLE_HAS_PERMISSION_ROLE_ID)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(RoleHasPermission::Table)
                    .name(FK_ROLE_HAS_PERMISSION_PERMISSION_ID)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(RoleHasPermission::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RoleHasPermission {
    Table,
    RoleId,
    PermissionId,
}
