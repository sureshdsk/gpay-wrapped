use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add transaction_hash column for deduplication
        manager
            .alter_table(
                Table::alter()
                    .table(Transactions::Table)
                    .add_column(ColumnDef::new(Transactions::TransactionHash).string().null())
                    .to_owned(),
            )
            .await?;

        // Create unique index on transaction_hash to prevent duplicates
        // Note: We use a unique index instead of a unique constraint to allow NULL values
        // (PostgreSQL treats NULL as distinct in unique constraints)
        manager
            .create_index(
                Index::create()
                    .name("idx_transactions_hash_unique")
                    .table(Transactions::Table)
                    .col(Transactions::TransactionHash)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Create composite index for efficient duplicate checking
        // This helps find duplicates by key fields even without hash
        manager
            .create_index(
                Index::create()
                    .name("idx_transactions_dedup")
                    .table(Transactions::Table)
                    .col(Transactions::UserId)
                    .col(Transactions::AccountId)
                    .col(Transactions::TransactionDate)
                    .col(Transactions::Amount)
                    .col(Transactions::Description)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop indexes
        manager
            .drop_index(
                Index::drop()
                    .name("idx_transactions_hash_unique")
                    .table(Transactions::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_transactions_dedup")
                    .table(Transactions::Table)
                    .to_owned(),
            )
            .await?;

        // Remove column
        manager
            .alter_table(
                Table::alter()
                    .table(Transactions::Table)
                    .drop_column(Transactions::TransactionHash)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Transactions {
    Table,
    TransactionHash,
    UserId,
    AccountId,
    TransactionDate,
    Amount,
    Description,
}
