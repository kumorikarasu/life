use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("sim"))
                    .add_column(
                        ColumnDef::new(Alias::new("user_id"))
                            .integer()
                            .not_null()
                            .default(1)
                    )
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_sim_user")
                    .from_tbl(Alias::new("sim"))
                    .from_col(Alias::new("user_id"))
                    .to_tbl(Alias::new("users"))
                    .to_col(Alias::new("id"))
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("sim"))
                    .drop_column(Alias::new("user_id"))
                    .to_owned(),
            )
            .await
    }
}
