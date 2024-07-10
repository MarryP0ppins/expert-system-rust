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
            
            ALTER TABLE \"public\".\"likes\" 
            ALTER COLUMN \"id\" DROP DEFAULT;

            CREATE SEQUENCE \"public\".\"likes_id_seq\"
            INCREMENT 1
            START 1
            MINVALUE 1
            MAXVALUE 2147483647
            CACHE 1
            OWNED BY \"likes\".\"id\";

            ALTER SEQUENCE \"public\".\"likes_id_seq\"
            OWNER TO postgres;
            ",
        )
        .await?;
        Ok(())
    }

}
