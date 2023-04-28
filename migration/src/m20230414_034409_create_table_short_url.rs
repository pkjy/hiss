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
                    .table(ShortUrl::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ShortUrl::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ShortUrl::CompanyId).string())
                    .col(ColumnDef::new(ShortUrl::ShortDomain).string().not_null())
                    .col(ColumnDef::new(ShortUrl::ShortCode).string().not_null())
                    .col(ColumnDef::new(ShortUrl::OriginalUrl).string().not_null())
                    .col(ColumnDef::new(ShortUrl::ExpiredAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(ShortUrl::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum ShortUrl {
    Table,
    Id,
    CompanyId,
    ShortDomain,
    ShortCode,
    OriginalUrl,
    ExpiredAt,
}
