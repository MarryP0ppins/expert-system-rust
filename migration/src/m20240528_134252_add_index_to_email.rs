use entity::users::Entity;
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
                    .name("idx-email")
                    .table(Entity)
                    .col(Alias::new("email"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Entity)
                    .drop_column(Alias::new("password_reset_token"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_index(Index::drop().name("idx-email").table(Entity).to_owned())
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Entity)
                    .add_column(
                        ColumnDef::new(Alias::new("password_reset_token"))
                            .string()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }
}
