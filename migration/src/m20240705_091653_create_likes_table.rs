use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();

        db.execute_unprepared(
            "
            CREATE TABLE \"public\".\"likes\" (
            \"id\" int4 NOT NULL,
            \"user_id\" int4 NOT NULL,
            \"system_id\" int4 NOT NULL,
            PRIMARY KEY (\"user_id\", \"system_id\"),
            CONSTRAINT \"user_id_user\" FOREIGN KEY (\"user_id\") REFERENCES \"public\".\"users\" (\"id\") ON DELETE CASCADE ON UPDATE NO ACTION,
            CONSTRAINT \"system_id_system\" FOREIGN KEY (\"system_id\") REFERENCES \"public\".\"systems\" (\"id\") ON DELETE CASCADE ON UPDATE NO ACTION
            )
            ;
            ",
        )
        .await?;
        Ok(())
    }
}
