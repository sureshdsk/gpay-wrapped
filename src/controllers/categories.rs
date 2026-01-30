use crate::models::{
    _entities::users,
    categories::{self, CreateCategoryParams, UpdateCategoryParams},
};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CategoryResponse {
    pub id: i32,
    pub name: String,
    pub color: String,
    pub icon: Option<String>,
    pub category_type: String,
    pub is_system: bool,
}

impl From<categories::Model> for CategoryResponse {
    fn from(cat: categories::Model) -> Self {
        Self {
            id: cat.id,
            name: cat.name,
            color: cat.color,
            icon: cat.icon,
            category_type: cat.category_type,
            is_system: cat.is_system,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub color: String,
    pub icon: Option<String>,
    pub category_type: String, // "income" or "expense"
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
}

/// List all categories for the current user (including system categories)
#[debug_handler]
async fn list_categories(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let cats = categories::Model::find_by_user(&ctx.db, user.id).await?;

    let response: Vec<CategoryResponse> = cats.into_iter().map(CategoryResponse::from).collect();

    format::json(response)
}

/// Get categories by type
#[debug_handler]
async fn list_categories_by_type(
    auth: auth::JWT,
    Path(category_type): Path<String>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let cats = categories::Model::find_by_type(&ctx.db, user.id, &category_type).await?;

    let response: Vec<CategoryResponse> = cats.into_iter().map(CategoryResponse::from).collect();

    format::json(response)
}

/// Create a new category
#[debug_handler]
async fn create_category(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(req): Json<CreateCategoryRequest>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // Validate category_type
    if req.category_type != "income" && req.category_type != "expense" {
        return Err(Error::BadRequest(
            "category_type must be 'income' or 'expense'".to_string(),
        ));
    }

    let params = CreateCategoryParams {
        name: req.name,
        color: req.color,
        icon: req.icon,
        category_type: req.category_type,
    };

    let cat = categories::Model::create(&ctx.db, user.id, &params).await?;

    format::json(CategoryResponse::from(cat))
}

/// Update a category
#[debug_handler]
async fn update_category(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(req): Json<UpdateCategoryRequest>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let params = UpdateCategoryParams {
        name: req.name,
        color: req.color,
        icon: req.icon,
    };

    let cat = categories::Model::update_category(&ctx.db, id, user.id, &params).await?;

    format::json(CategoryResponse::from(cat))
}

/// Delete a category
#[debug_handler]
async fn delete_category(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    categories::Model::delete_category(&ctx.db, id, user.id).await?;

    format::json(serde_json::json!({"status": "deleted"}))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/v1/categories")
        .add("/", get(list_categories))
        .add("/", post(create_category))
        .add("/type/{category_type}", get(list_categories_by_type))
        .add("/{id}", put(update_category))
        .add("/{id}", delete(delete_category))
}
