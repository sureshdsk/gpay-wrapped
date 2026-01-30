use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add unique constraint on pid (the external/public transaction ID)
        // This ensures each transaction has a globally unique identifier
        manager
            .create_index(
                Index::create()
                    .name("idx_transactions_pid_unique")
                    .table(Transactions::Table)
                    .col(Transactions::Pid)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Add unique constraint on reference_number for bank transaction IDs
        // This ensures the same bank transaction ID (UPI, IMPS, check number, etc.)
        // doesn't appear in multiple accounts
        // Note: Only non-null values are constrained (multiple nulls allowed)
        manager
            .create_index(
                Index::create()
                    .name("idx_transactions_reference_unique")
                    .table(Transactions::Table)
                    .col(Transactions::ReferenceNumber)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop unique indexes
        manager
            .drop_index(
                Index::drop()
                    .name("idx_transactions_pid_unique")
                    .table(Transactions::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_transactions_reference_unique")
                    .table(Transactions::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Transactions {
    Table,
    Pid,
    ReferenceNumber,
}
