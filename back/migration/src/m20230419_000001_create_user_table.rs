use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::GoogleId).string().not_null().unique_key())
                    .col(ColumnDef::new(Users::Email).string().not_null())
                    .col(ColumnDef::new(Users::Name).string().not_null())
                    .col(ColumnDef::new(Users::GivenName).string())
                    .col(ColumnDef::new(Users::FamilyName).string())
                    .col(ColumnDef::new(Users::Picture).string())
                    .col(ColumnDef::new(Users::Locale).string())
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Users::UpdatedAt).timestamp())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Users {
    Table,
    Id,
    GoogleId,
    Email,
    Name,
    GivenName,
    FamilyName,
    Picture,
    Locale,
    CreatedAt,
    UpdatedAt,
}