use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::{Deserialize, Serialize};

pub use super::_entities::feature_definitions::{self, ActiveModel, Entity, Model};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateFeatureParams {
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub default_enabled: bool,
    pub is_premium: bool,
    pub sort_order: i32,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for super::_entities::feature_definitions::ActiveModel {}

impl Model {
    /// Find a feature definition by key
    pub async fn find_by_key(db: &DatabaseConnection, key: &str) -> ModelResult<Self> {
        let feature = feature_definitions::Entity::find()
            .filter(
                model::query::condition()
                    .eq(feature_definitions::Column::Key, key)
                    .build(),
            )
            .one(db)
            .await?;
        feature.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Get all feature definitions
    pub async fn find_all(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        let features = feature_definitions::Entity::find()
            .order_by_asc(feature_definitions::Column::SortOrder)
            .all(db)
            .await?;
        Ok(features)
    }

    /// Get all feature definitions by category
    pub async fn find_by_category(db: &DatabaseConnection, category: &str) -> ModelResult<Vec<Self>> {
        let features = feature_definitions::Entity::find()
            .filter(
                model::query::condition()
                    .eq(feature_definitions::Column::Category, category)
                    .build(),
            )
            .order_by_asc(feature_definitions::Column::SortOrder)
            .all(db)
            .await?;
        Ok(features)
    }
}
