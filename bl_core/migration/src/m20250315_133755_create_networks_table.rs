use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Networks::Table)
                    .if_not_exists()
                    .col(pk_auto(Networks::Id))
                    .col(string(Networks::Name))
                    .col(string(Networks::Subnet))
                    .col(string(Networks::Driver))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Networks::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Networks {
    Table,
    Id,
    Name,
    Subnet,
    Driver,
}
