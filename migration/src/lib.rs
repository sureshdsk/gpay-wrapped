#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;
mod m20240101_000002_create_feature_definitions;
mod m20240101_000003_create_user_feature_flags;
mod m20240101_000004_create_categories;
mod m20240101_000005_create_bank_accounts;
mod m20240101_000006_create_statements;
mod m20240101_000007_create_transactions;
mod m20240101_000008_add_bank_columns;
mod m20240101_000009_add_transaction_deduplication;
mod m20240101_000010_add_transaction_uniqueness;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20240101_000002_create_feature_definitions::Migration),
            Box::new(m20240101_000003_create_user_feature_flags::Migration),
            Box::new(m20240101_000004_create_categories::Migration),
            Box::new(m20240101_000005_create_bank_accounts::Migration),
            Box::new(m20240101_000006_create_statements::Migration),
            Box::new(m20240101_000007_create_transactions::Migration),
            Box::new(m20240101_000008_add_bank_columns::Migration),
            Box::new(m20240101_000009_add_transaction_deduplication::Migration),
            Box::new(m20240101_000010_add_transaction_uniqueness::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
