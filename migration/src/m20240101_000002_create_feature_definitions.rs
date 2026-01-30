use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "feature_definitions",
            &[
                ("id", ColType::PkAuto),
                ("key", ColType::StringUniq),        // unique feature identifier e.g. "dark_mode"
                ("name", ColType::String),            // display name e.g. "Dark Mode"
                ("description", ColType::TextNull),   // optional description
                ("category", ColType::String),        // category for grouping e.g. "ui", "finance"
                ("default_enabled", ColType::BooleanWithDefault(false)),  // default state for new users
                ("is_premium", ColType::BooleanWithDefault(false)),       // requires premium subscription
                ("sort_order", ColType::IntegerWithDefault(0)),        // display order
            ],
            &[],
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "feature_definitions").await?;
        Ok(())
    }
}
