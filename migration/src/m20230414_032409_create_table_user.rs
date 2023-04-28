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
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            // .integer()
                            .string()
                            .not_null()
                            // .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::UserName).string())
                    .col(ColumnDef::new(User::UserEmail).string())
                    .col(ColumnDef::new(User::UserPassword).string().not_null())
                    .col(ColumnDef::new(User::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                    .col(ColumnDef::new(User::UpdatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                    .col(ColumnDef::new(User::DeletedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden



#[derive(Iden)]
enum User{
  Table,
  Id ,
  UserName,
  UserEmail,
  UserPassword,
  CreatedAt ,
  UpdatedAt,
  DeletedAt 
}