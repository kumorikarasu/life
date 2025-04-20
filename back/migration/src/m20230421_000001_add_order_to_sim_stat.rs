use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add order column to sim_stat table
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("sim_stat"))
                    .add_column(
                        ColumnDef::new(Alias::new("order_index"))
                            .integer()
                            .not_null()
                            .default(0)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the order_index column
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("sim_stat"))
                    .drop_column(Alias::new("order_index"))
                    .to_owned(),
            )
            .await
    }
}