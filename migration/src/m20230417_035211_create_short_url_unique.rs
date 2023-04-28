use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_index(
                Index::create()
                    .name("unique-short-domain-and-hash")
                    .if_not_exists()
                    .table(ShortUrl::Table)
                    .col(ShortUrl::ShortCode)
                    .col(ShortUrl::ShortDomain)
                    .unique()
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_index(
              Index::drop()
                    .name("unique-short-domain-and-hash")
                    .to_owned(),
            )
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum ShortUrl {
    Table,
    ShortDomain,
    ShortCode,
}
