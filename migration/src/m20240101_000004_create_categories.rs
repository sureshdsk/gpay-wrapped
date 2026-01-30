use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "categories",
            &[
                ("id", ColType::PkAuto),
                ("user_id", ColType::IntegerNull),
                ("name", ColType::String),
                ("color", ColType::String),
                ("icon", ColType::StringNull),
                ("category_type", ColType::String),
                ("is_system", ColType::BooleanWithDefault(false)),
            ],
            &[],
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "categories").await?;
        Ok(())
    }
}
