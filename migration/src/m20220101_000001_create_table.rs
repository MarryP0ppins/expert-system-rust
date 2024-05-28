use entity::users::Entity;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .alter_table(
                Table::alter()
                    .table(Entity)
                    .add_column(
                        ColumnDef::new(Alias::new("verified"))
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .add_column(
                        ColumnDef::new(Alias::new("verification_code"))
                            .string()
                            .null(),
                    )
                    .add_column(
                        ColumnDef::new(Alias::new("password_reset_token"))
                            .string()
                            .null(),
                    )
                    .add_column(
                        ColumnDef::new(Alias::new("password_reset_at"))
                            .date_time()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .alter_table(
                Table::alter()
                    .table(Entity)
                    .drop_column(Alias::new("verified"))
                    .drop_column(Alias::new("verification_code"))
                    .drop_column(Alias::new("password_reset_token"))
                    .drop_column(Alias::new("password_reset_at"))
                    .to_owned(),
            )
            .await
    }
}
