pub use sea_orm_migration::prelude::*;

mod m20240606_000001_create_all;
mod m20240703_143043_add_stars_to_systems;
mod m20240705_091653_create_likes_table;
mod m20240705_113436_update_likes;
mod m20240705_114013_update_likes;
mod m20240705_114453_update_likes;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240606_000001_create_all::Migration),
            Box::new(m20240703_143043_add_stars_to_systems::Migration),
            Box::new(m20240705_091653_create_likes_table::Migration),
            Box::new(m20240705_113436_update_likes::Migration),
            Box::new(m20240705_114013_update_likes::Migration),
            Box::new(m20240705_114453_update_likes::Migration),
        ]
    }
}
