use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "user_feature_flags",
            &[
                ("id", ColType::PkAuto),
                ("user_id", ColType::Integer),
                ("feature_id", ColType::Integer),
                ("enabled", ColType::BooleanWithDefault(true)),
            ],
            &[],
        )
        .await?;

        // Add unique constraint on user_id + feature_id
        m.create_index(
            Index::create()
                .name("idx_user_feature_unique")
                .table(UserFeatureFlags::Table)
                .col(UserFeatureFlags::UserId)
                .col(UserFeatureFlags::FeatureId)
                .unique()
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "user_feature_flags").await?;
        Ok(())
    }
}

#[derive(Iden)]
enum UserFeatureFlags {
    Table,
    UserId,
    FeatureId,
}
