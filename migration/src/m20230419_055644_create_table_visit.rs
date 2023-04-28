use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Visit::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Visit::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Visit::Ua).string().not_null())
                    .col(ColumnDef::new(Visit::Ip).string().not_null())
                    .col(ColumnDef::new(Visit::ShortUrlId).string())
                    .col(ColumnDef::new(Visit::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Visit::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Visit {
    Table,
    Id,
    Ua,
    Ip,
    ShortUrlId,
    CreatedAt
}
