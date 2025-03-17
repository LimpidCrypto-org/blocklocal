use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20250315_133755_create_networks_table::Networks, m20250315_133828_create_nodes_table::Nodes,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

const PK_NETWORK_HAS_NODE_NETWORK_ID_NODE_ID: &str = "PK_networkHasNode_networkId_nodeId";
const FK_NETWORK_HAS_NODE_NETWORK_ID: &str = "FK_networkHasNode_networkId";
const FK_NETWORK_HAS_NODE_NODE_ID: &str = "FK_networkHasNode_nodeId";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(NetworkHasNode::Table)
                    .if_not_exists()
                    .col(integer(NetworkHasNode::NetworkId))
                    .col(integer(NetworkHasNode::NodeId))
                    .col(integer(NetworkHasNode::Replicas).default(1))
                    .primary_key(
                        Index::create()
                            .col(NetworkHasNode::NetworkId)
                            .col(NetworkHasNode::NodeId)
                            .name(PK_NETWORK_HAS_NODE_NETWORK_ID_NODE_ID),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(NetworkHasNode::Table, NetworkHasNode::NetworkId)
                    .to(Networks::Table, Networks::Id)
                    .name(FK_NETWORK_HAS_NODE_NETWORK_ID)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(NetworkHasNode::Table, NetworkHasNode::NodeId)
                    .to(Nodes::Table, Nodes::Id)
                    .name(FK_NETWORK_HAS_NODE_NODE_ID)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(NetworkHasNode::Table)
                    .name(FK_NETWORK_HAS_NODE_NODE_ID)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(NetworkHasNode::Table)
                    .name(FK_NETWORK_HAS_NODE_NETWORK_ID)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(NetworkHasNode::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum NetworkHasNode {
    Table,
    NetworkId,
    NodeId,
    Replicas,
}
