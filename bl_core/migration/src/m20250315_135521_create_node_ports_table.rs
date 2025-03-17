use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250315_133828_create_nodes_table::Nodes;

#[derive(DeriveMigrationName)]
pub struct Migration;

const PK_NODE_PORTS_NODE_ID_CONTAINER_PORT_HOST_PORT: &str =
    "PK_nodePorts_nodeId_containerPort_hostPort";
const FK_NODE_PORTS_NODE_ID: &str = "FK_nodePorts_nodeId";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(NodePorts::Table)
                    .if_not_exists()
                    .col(integer(NodePorts::NodeId))
                    .col(integer(NodePorts::ContainerPort))
                    .col(integer(NodePorts::HostPort))
                    .col(string_null(NodePorts::Name).default("NULL"))
                    .primary_key(
                        Index::create()
                            .col(NodePorts::NodeId)
                            .col(NodePorts::ContainerPort)
                            .col(NodePorts::HostPort)
                            .name(PK_NODE_PORTS_NODE_ID_CONTAINER_PORT_HOST_PORT),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(NodePorts::Table, NodePorts::NodeId)
                    .to(Nodes::Table, Nodes::Id)
                    .name(FK_NODE_PORTS_NODE_ID)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(NodePorts::Table)
                    .name(FK_NODE_PORTS_NODE_ID)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(NodePorts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum NodePorts {
    Table,
    NodeId,
    ContainerPort,
    HostPort,
    Name,
}
