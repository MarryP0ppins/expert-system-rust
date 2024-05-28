pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240528_124555_add_index_to_verify_code;
mod m20240528_134252_add_index_to_email;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240528_124555_add_index_to_verify_code::Migration),
            Box::new(m20240528_134252_add_index_to_email::Migration),
        ]
    }
}
