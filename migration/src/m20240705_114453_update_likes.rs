use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "
            
            ALTER TABLE \"public\".\"likes\" 
            ALTER COLUMN \"id\" SET DEFAULT nextval('likes_id_seq'::regclass);
            ",
        )
        .await?;
        Ok(())
    }
}
