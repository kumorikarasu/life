use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Sim::Table)
                    .if_not_exists()
                    .col(pk_auto(Sim::Id))
                    .col(string(Sim::Name))
                    .to_owned(),
            )
            .await?;


        manager
            .create_table(
                Table::create()
                    .table(SimStat::Table)
                    .if_not_exists()
                    .col(pk_auto(SimStat::Id))
                    .col(integer(SimStat::SimId))
                    .col(string(SimStat::Name))
                    .col(integer(SimStat::Value))
                    .to_owned(),
            )
            .await?;

        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk_stat_id")
                .from_tbl(SimStat::Table)
                .from_col(SimStat::SimId)
                .to_tbl(Sim::Table)
                .to_col(Sim::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned(),
        ).await

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SimStat::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Sim::Table).to_owned())
            .await

    }
}

#[derive(DeriveIden)]
enum Sim {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum SimStat {
    Table,
    Id,
    SimId,
    Name,
    Value,
}
