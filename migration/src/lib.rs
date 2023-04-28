pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table_company;
mod m20230414_032409_create_table_user;
mod m20230414_033627_create_table_company_user;
mod m20230414_033849_create_table_short_domain;
mod m20230414_034409_create_table_short_url;
mod m20230417_035211_create_short_url_unique;
mod m20230419_055644_create_table_visit;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table_company::Migration),
            Box::new(m20230414_032409_create_table_user::Migration),
            Box::new(m20230414_033627_create_table_company_user::Migration),
            Box::new(m20230414_033849_create_table_short_domain::Migration),
            Box::new(m20230414_034409_create_table_short_url::Migration),
            Box::new(m20230417_035211_create_short_url_unique::Migration),
            Box::new(m20230419_055644_create_table_visit::Migration),
        ]
    }
}
