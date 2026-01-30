use crate::models::{
    _entities::{feature_definitions, user_feature_flags, users},
    user_feature_flags as user_feature_model,
};
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct FeatureResponse {
    pub id: i32,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub is_premium: bool,
    pub sort_order: i32,
}

#[derive(Debug, Serialize)]
pub struct UserFeatureResponse {
    pub id: i32,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub is_premium: bool,
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct SetFeatureParams {
    pub enabled: bool,
}

/// List all available feature definitions
#[debug_handler]
async fn list_features(State(ctx): State<AppContext>) -> Result<Response> {
    let features = feature_definitions::Entity::find()
        .order_by_asc(feature_definitions::Column::SortOrder)
        .all(&ctx.db)
        .await?;

    let response: Vec<FeatureResponse> = features
        .into_iter()
        .map(|f| FeatureResponse {
            id: f.id,
            key: f.key,
            name: f.name,
            description: f.description,
            category: f.category,
            is_premium: f.is_premium,
            sort_order: f.sort_order,
        })
        .collect();

    format::json(response)
}

/// Get the current user's feature flags with their enabled status
#[debug_handler]
async fn get_user_features(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // Get all feature definitions
    let features = feature_definitions::Entity::find()
        .order_by_asc(feature_definitions::Column::SortOrder)
        .all(&ctx.db)
        .await?;

    // Get user's feature overrides
    let user_flags = user_feature_flags::Entity::find()
        .filter(user_feature_flags::Column::UserId.eq(user.id))
        .all(&ctx.db)
        .await?;

    // Map features with user's enabled status
    let response: Vec<UserFeatureResponse> = features
        .into_iter()
        .map(|f| {
            let user_override = user_flags.iter().find(|uf| uf.feature_id == f.id);
            let enabled = user_override
                .map(|uf| uf.enabled)
                .unwrap_or(f.default_enabled);

            UserFeatureResponse {
                id: f.id,
                key: f.key,
                name: f.name,
                description: f.description,
                category: f.category,
                is_premium: f.is_premium,
                enabled,
            }
        })
        .collect();

    format::json(response)
}

/// Enable a feature for the current user
#[debug_handler]
async fn enable_feature(
    auth: auth::JWT,
    Path(feature_id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // Verify feature exists
    let _feature = feature_definitions::Entity::find_by_id(feature_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    // Upsert user feature flag
    user_feature_model::Model::set_feature(&ctx.db, user.id, feature_id, true).await?;

    format::json(serde_json::json!({"status": "enabled"}))
}

/// Disable a feature for the current user
#[debug_handler]
async fn disable_feature(
    auth: auth::JWT,
    Path(feature_id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // Verify feature exists
    let _feature = feature_definitions::Entity::find_by_id(feature_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    // Upsert user feature flag
    user_feature_model::Model::set_feature(&ctx.db, user.id, feature_id, false).await?;

    format::json(serde_json::json!({"status": "disabled"}))
}

/// Toggle a feature for the current user
#[debug_handler]
async fn toggle_feature(
    auth: auth::JWT,
    Path(feature_id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<SetFeatureParams>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // Verify feature exists
    let _feature = feature_definitions::Entity::find_by_id(feature_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    // Upsert user feature flag
    user_feature_model::Model::set_feature(&ctx.db, user.id, feature_id, params.enabled).await?;

    format::json(serde_json::json!({"status": if params.enabled { "enabled" } else { "disabled" }}))
}

/// Check if a specific feature is enabled for the current user
#[debug_handler]
async fn check_feature(
    auth: auth::JWT,
    Path(feature_key): Path<String>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let enabled =
        user_feature_model::Model::is_feature_enabled(&ctx.db, user.id, &feature_key).await?;

    format::json(serde_json::json!({"feature": feature_key, "enabled": enabled}))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/v1")
        .add("/features", get(list_features))
        .add("/user/features", get(get_user_features))
        .add("/user/features/{feature_id}/enable", post(enable_feature))
        .add("/user/features/{feature_id}/disable", post(disable_feature))
        .add("/user/features/{feature_id}", put(toggle_feature))
        .add("/user/features/check/{feature_key}", get(check_feature))
}
