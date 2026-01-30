use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add bank_name column (SQLite requires one column per ALTER TABLE)
        manager
            .alter_table(
                Table::alter()
                    .table(Statements::Table)
                    .add_column(ColumnDef::new(Statements::BankName).string().null())
                    .to_owned(),
            )
            .await?;

        // Add detection_confidence column
        manager
            .alter_table(
                Table::alter()
                    .table(Statements::Table)
                    .add_column(ColumnDef::new(Statements::DetectionConfidence).integer().null())
                    .to_owned(),
            )
            .await?;

        // Add parser_used column
        manager
            .alter_table(
                Table::alter()
                    .table(Statements::Table)
                    .add_column(ColumnDef::new(Statements::ParserUsed).string().null())
                    .to_owned(),
            )
            .await?;

        // Add index for bank_name
        manager
            .create_index(
                Index::create()
                    .name("idx_statements_bank_name")
                    .table(Statements::Table)
                    .col(Statements::BankName)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop index
        manager
            .drop_index(
                Index::drop()
                    .name("idx_statements_bank_name")
                    .table(Statements::Table)
                    .to_owned(),
            )
            .await?;

        // Remove columns (each in separate statement for SQLite compatibility)
        manager
            .alter_table(
                Table::alter()
                    .table(Statements::Table)
                    .drop_column(Statements::BankName)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Statements::Table)
                    .drop_column(Statements::DetectionConfidence)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Statements::Table)
                    .drop_column(Statements::ParserUsed)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Statements {
    Table,
    BankName,
    DetectionConfidence,
    ParserUsed,
}
