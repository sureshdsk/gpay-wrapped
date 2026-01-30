use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "statements",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::Uuid),
                ("user_id", ColType::Integer),
                ("account_id", ColType::IntegerNull),
                ("filename", ColType::String),
                ("file_path", ColType::String),
                ("file_size", ColType::Integer),
                ("file_type", ColType::String),
                ("status", ColType::StringWithDefault("pending".to_string())),
                ("statement_date", ColType::DateNull),
                ("start_date", ColType::DateNull),
                ("end_date", ColType::DateNull),
                ("transaction_count", ColType::IntegerWithDefault(0)),
                ("error_message", ColType::TextNull),
                ("processed_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;

        // Add indexes
        m.create_index(
            Index::create()
                .name("idx_statements_user_id")
                .table(Statements::Table)
                .col(Statements::UserId)
                .to_owned(),
        )
        .await?;

        m.create_index(
            Index::create()
                .name("idx_statements_status")
                .table(Statements::Table)
                .col(Statements::Status)
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "statements").await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Statements {
    Table,
    UserId,
    Status,
}
