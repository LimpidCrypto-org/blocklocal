use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20220101_000001_create_users_table::Users, m20250314_220417_create_profiles_table::Profiles,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

const FK_PROFILE_HAS_USER_PROFILE_ID: &str = "FK_profileHasUser_profileId";
const FK_PROFILE_HAS_USER_USER_ID: &str = "FK_profileHasUser_userId";
const PK_PROFILE_HAS_USER_PROFILE_ID_USER_ID: &str = "PK_profileHasUser_profileId_userId";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProfileHasUser::Table)
                    .if_not_exists()
                    .col(integer(ProfileHasUser::ProfileId))
                    .col(integer(ProfileHasUser::UserId))
                    .primary_key(
                        Index::create()
                            .col(ProfileHasUser::ProfileId)
                            .col(ProfileHasUser::UserId)
                            .name(PK_PROFILE_HAS_USER_PROFILE_ID_USER_ID),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(ProfileHasUser::Table, ProfileHasUser::ProfileId)
                    .to(Profiles::Table, Profiles::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .name(FK_PROFILE_HAS_USER_PROFILE_ID)
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(ProfileHasUser::Table, ProfileHasUser::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .name(FK_PROFILE_HAS_USER_USER_ID)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name(FK_PROFILE_HAS_USER_PROFILE_ID)
                    .table(ProfileHasUser::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name(FK_PROFILE_HAS_USER_USER_ID)
                    .table(ProfileHasUser::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(ProfileHasUser::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ProfileHasUser {
    Table,
    ProfileId,
    UserId,
}
