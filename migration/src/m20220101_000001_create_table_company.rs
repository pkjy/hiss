use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .create_table(
                Table::create()
                    .table(Company::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Company::Id)
                            // .integer()
                            .string()
                            .not_null()
                            // .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Company::CompanyName).string().not_null())
                    .col(ColumnDef::new(Company::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                    .col(ColumnDef::new(Company::UpdatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                    .col(ColumnDef::new(Company::DeletedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(Company::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Company {
  Table,
  Id,
  CompanyName,
  CreatedAt ,
  UpdatedAt,
  DeletedAt 
}