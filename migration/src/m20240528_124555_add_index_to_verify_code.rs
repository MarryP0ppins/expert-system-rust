use sea_orm_migration::prelude::*;

use entity::users::Entity;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_index(
                Index::create()
                    .name("idx-verification_code")
                    .table(Entity)
                    .col(Alias::new("verification_code"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_index(
                Index::drop()
                    .name("idx-verification_code")
                    .table(Entity)
                    .to_owned(),
            )
            .await
    }
}
