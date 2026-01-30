use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "transactions",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::Uuid),
                ("user_id", ColType::Integer),
                ("account_id", ColType::Integer),
                ("category_id", ColType::IntegerNull),
                ("statement_id", ColType::IntegerNull),
                ("transaction_date", ColType::Date),
                ("posted_date", ColType::DateNull),
                ("description", ColType::String),
                ("original_description", ColType::StringNull),
                ("amount", ColType::Decimal),
                ("transaction_type", ColType::String),
                ("status", ColType::StringWithDefault("posted".to_string())),
                ("merchant_name", ColType::StringNull),
                ("reference_number", ColType::StringNull),
                ("notes", ColType::TextNull),
                ("is_recurring", ColType::BooleanWithDefault(false)),
                ("is_excluded", ColType::BooleanWithDefault(false)),
            ],
            &[],
        )
        .await?;

        // Add indexes
        m.create_index(
            Index::create()
                .name("idx_transactions_user_id")
                .table(Transactions::Table)
                .col(Transactions::UserId)
                .to_owned(),
        )
        .await?;

        m.create_index(
            Index::create()
                .name("idx_transactions_account_id")
                .table(Transactions::Table)
                .col(Transactions::AccountId)
                .to_owned(),
        )
        .await?;

        m.create_index(
            Index::create()
                .name("idx_transactions_transaction_date")
                .table(Transactions::Table)
                .col(Transactions::TransactionDate)
                .to_owned(),
        )
        .await?;

        m.create_index(
            Index::create()
                .name("idx_transactions_category_id")
                .table(Transactions::Table)
                .col(Transactions::CategoryId)
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "transactions").await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Transactions {
    Table,
    UserId,
    AccountId,
    TransactionDate,
    CategoryId,
}
