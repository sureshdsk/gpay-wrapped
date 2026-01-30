use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "bank_accounts",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::Uuid),
                ("user_id", ColType::Integer),
                ("name", ColType::String),
                ("account_type", ColType::String),
                ("institution", ColType::StringNull),
                ("account_number_last4", ColType::StringNull),
                ("currency", ColType::StringWithDefault("USD".to_string())),
                ("current_balance", ColType::Decimal),
                ("available_balance", ColType::DecimalNull),
                ("color", ColType::StringWithDefault("#3d84f5".to_string())),
                ("is_active", ColType::BooleanWithDefault(true)),
                ("last_synced_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;

        // Add index for user_id
        m.create_index(
            Index::create()
                .name("idx_bank_accounts_user_id")
                .table(BankAccounts::Table)
                .col(BankAccounts::UserId)
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "bank_accounts").await?;
        Ok(())
    }
}

#[derive(Iden)]
enum BankAccounts {
    Table,
    UserId,
}
