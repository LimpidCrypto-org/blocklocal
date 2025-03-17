use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

const UQ_PERMISSIONS_NAME: &str = "UQ_permissions_name";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Permissions::Table)
                    .if_not_exists()
                    .col(pk_auto(Permissions::Id))
                    .col(string(Permissions::Name).not_null())
                    .col(
                        string(Permissions::CreatedAt)
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Permissions::Table)
                    .col(Permissions::Name)
                    .unique()
                    .name(UQ_PERMISSIONS_NAME)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .table(Permissions::Table)
                    .name(UQ_PERMISSIONS_NAME)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(Permissions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Permissions {
    Table,
    Id,
    Name,
    CreatedAt,
}
