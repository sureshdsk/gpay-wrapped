use loco_rs::prelude::*;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};

pub use super::_entities::categories::{self, ActiveModel, Entity, Model};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCategoryParams {
    pub name: String,
    pub color: String,
    pub icon: Option<String>,
    pub category_type: String, // "income" or "expense"
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateCategoryParams {
    pub name: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for super::_entities::categories::ActiveModel {}

impl Model {
    /// Find all categories for a user (including system categories)
    pub async fn find_by_user(db: &DatabaseConnection, user_id: i32) -> ModelResult<Vec<Self>> {
        use sea_orm::QueryOrder;
        let cats = categories::Entity::find()
            .filter(
                sea_orm::Condition::any()
                    .add(categories::Column::UserId.eq(user_id))
                    .add(categories::Column::IsSystem.eq(true)),
            )
            .order_by_asc(categories::Column::Name)
            .all(db)
            .await?;
        Ok(cats)
    }

    /// Find categories by type for a user
    pub async fn find_by_type(
        db: &DatabaseConnection,
        user_id: i32,
        category_type: &str,
    ) -> ModelResult<Vec<Self>> {
        let cats = categories::Entity::find()
            .filter(
                sea_orm::Condition::all()
                    .add(
                        sea_orm::Condition::any()
                            .add(categories::Column::UserId.eq(user_id))
                            .add(categories::Column::IsSystem.eq(true)),
                    )
                    .add(categories::Column::CategoryType.eq(category_type)),
            )
            .all(db)
            .await?;
        Ok(cats)
    }

    /// Create a new user category
    pub async fn create(
        db: &DatabaseConnection,
        user_id: i32,
        params: &CreateCategoryParams,
    ) -> ModelResult<Self> {
        let active = ActiveModel {
            user_id: ActiveValue::Set(Some(user_id)),
            name: ActiveValue::Set(params.name.clone()),
            color: ActiveValue::Set(params.color.clone()),
            icon: ActiveValue::Set(params.icon.clone()),
            category_type: ActiveValue::Set(params.category_type.clone()),
            is_system: ActiveValue::Set(false),
            ..Default::default()
        };
        active.insert(db).await.map_err(ModelError::from)
    }

    /// Update a category (only non-system categories)
    pub async fn update_category(
        db: &DatabaseConnection,
        id: i32,
        user_id: i32,
        params: &UpdateCategoryParams,
    ) -> ModelResult<Self> {
        let cat = categories::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;

        // Check ownership and not system
        if cat.user_id != Some(user_id) || cat.is_system {
            return Err(ModelError::msg("Cannot modify this category"));
        }

        let mut active: ActiveModel = cat.into();
        if let Some(name) = &params.name {
            active.name = ActiveValue::Set(name.clone());
        }
        if let Some(color) = &params.color {
            active.color = ActiveValue::Set(color.clone());
        }
        if params.icon.is_some() {
            active.icon = ActiveValue::Set(params.icon.clone());
        }

        active.update(db).await.map_err(ModelError::from)
    }

    /// Delete a category (only non-system user categories)
    pub async fn delete_category(db: &DatabaseConnection, id: i32, user_id: i32) -> ModelResult<()> {
        let cat = categories::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;

        if cat.user_id != Some(user_id) || cat.is_system {
            return Err(ModelError::msg("Cannot delete this category"));
        }

        categories::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
