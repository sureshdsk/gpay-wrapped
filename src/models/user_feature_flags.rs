use loco_rs::prelude::*;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};

pub use super::_entities::user_feature_flags::{self, ActiveModel, Entity, Model};
use super::_entities::feature_definitions;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserFeatureResponse {
    pub feature_key: String,
    pub feature_name: String,
    pub enabled: bool,
    pub is_premium: bool,
    pub category: String,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for super::_entities::user_feature_flags::ActiveModel {}

impl Model {
    /// Find all feature flags for a user
    pub async fn find_by_user(db: &DatabaseConnection, user_id: i32) -> ModelResult<Vec<Self>> {
        let flags = user_feature_flags::Entity::find()
            .filter(
                model::query::condition()
                    .eq(user_feature_flags::Column::UserId, user_id)
                    .build(),
            )
            .all(db)
            .await?;
        Ok(flags)
    }

    /// Find a specific feature flag for a user
    pub async fn find_by_user_and_feature(
        db: &DatabaseConnection,
        user_id: i32,
        feature_id: i32,
    ) -> ModelResult<Self> {
        let flag = user_feature_flags::Entity::find()
            .filter(
                model::query::condition()
                    .eq(user_feature_flags::Column::UserId, user_id)
                    .eq(user_feature_flags::Column::FeatureId, feature_id)
                    .build(),
            )
            .one(db)
            .await?;
        flag.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Check if a feature is enabled for a user (considers default if no override exists)
    pub async fn is_feature_enabled(
        db: &DatabaseConnection,
        user_id: i32,
        feature_key: &str,
    ) -> ModelResult<bool> {
        // First get the feature definition
        let feature = feature_definitions::Entity::find()
            .filter(
                model::query::condition()
                    .eq(feature_definitions::Column::Key, feature_key)
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;

        // Try to find user override
        let user_flag = user_feature_flags::Entity::find()
            .filter(
                model::query::condition()
                    .eq(user_feature_flags::Column::UserId, user_id)
                    .eq(user_feature_flags::Column::FeatureId, feature.id)
                    .build(),
            )
            .one(db)
            .await?;

        // Return user override if exists, otherwise default
        Ok(user_flag
            .map(|f| f.enabled)
            .unwrap_or(feature.default_enabled))
    }

    /// Set a feature flag for a user (upsert)
    pub async fn set_feature(
        db: &DatabaseConnection,
        user_id: i32,
        feature_id: i32,
        enabled: bool,
    ) -> ModelResult<Self> {
        // Try to find existing flag
        let existing = user_feature_flags::Entity::find()
            .filter(
                model::query::condition()
                    .eq(user_feature_flags::Column::UserId, user_id)
                    .eq(user_feature_flags::Column::FeatureId, feature_id)
                    .build(),
            )
            .one(db)
            .await?;

        match existing {
            Some(flag) => {
                let mut active: ActiveModel = flag.into();
                active.enabled = ActiveValue::Set(enabled);
                active.update(db).await.map_err(ModelError::from)
            }
            None => {
                let active = ActiveModel {
                    user_id: ActiveValue::Set(user_id),
                    feature_id: ActiveValue::Set(feature_id),
                    enabled: ActiveValue::Set(enabled),
                    ..Default::default()
                };
                active.insert(db).await.map_err(ModelError::from)
            }
        }
    }
}
