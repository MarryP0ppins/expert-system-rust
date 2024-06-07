pub use sea_orm_migration::prelude::*;

mod m20240606_000001_create_all;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20240606_000001_create_all::Migration)]
    }
}
