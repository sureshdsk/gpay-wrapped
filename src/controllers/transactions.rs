use crate::models::{
    _entities::users,
    bank_accounts,
    transactions::{self, CreateTransactionParams, TransactionFilters, UpdateTransactionParams},
};
use chrono::NaiveDate;
use loco_rs::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    pub id: i32,
    pub pid: String,
    pub account_id: i32,
    pub category_id: Option<i32>,
    pub transaction_date: String,
    pub posted_date: Option<String>,
    pub description: String,
    pub original_description: Option<String>,
    pub amount: String,
    pub transaction_type: String,
    pub status: String,
    pub merchant_name: Option<String>,
    pub reference_number: Option<String>,
    pub notes: Option<String>,
    pub is_recurring: bool,
    pub is_excluded: bool,
    pub created_at: String,
}

impl From<transactions::Model> for TransactionResponse {
    fn from(txn: transactions::Model) -> Self {
        Self {
            id: txn.id,
            pid: txn.pid.to_string(),
            account_id: txn.account_id,
            category_id: txn.category_id,
            transaction_date: txn.transaction_date.to_string(),
            posted_date: txn.posted_date.map(|d| d.to_string()),
            description: txn.description,
            original_description: txn.original_description,
            amount: txn.amount.to_string(),
            transaction_type: txn.transaction_type,
            status: txn.status,
            merchant_name: txn.merchant_name,
            reference_number: txn.reference_number,
            notes: txn.notes,
            is_recurring: txn.is_recurring,
            is_excluded: txn.is_excluded,
            created_at: txn.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TransactionsListResponse {
    pub transactions: Vec<TransactionResponse>,
    pub page: u64,
    pub per_page: u64,
}

#[derive(Debug, Deserialize)]
pub struct ListTransactionsQuery {
    pub account_id: Option<i32>,
    pub category_id: Option<i32>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub transaction_type: Option<String>,
    pub search: Option<String>,
    pub min_amount: Option<String>,
    pub max_amount: Option<String>,
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTransactionRequest {
    pub account_id: i32,
    pub category_id: Option<i32>,
    pub transaction_date: String,
    pub posted_date: Option<String>,
    pub description: String,
    pub original_description: Option<String>,
    pub amount: String,
    pub transaction_type: String,
    pub merchant_name: Option<String>,
    pub reference_number: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTransactionRequest {
    pub category_id: Option<i32>,
    pub description: Option<String>,
    pub merchant_name: Option<String>,
    pub notes: Option<String>,
    pub is_recurring: Option<bool>,
    pub is_excluded: Option<bool>,
}

/// List transactions with filters
#[debug_handler]
async fn list_transactions(
    auth: auth::JWT,
    Query(query): Query<ListTransactionsQuery>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let start_date = query
        .start_date
        .map(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d"))
        .transpose()
        .map_err(|_| Error::BadRequest("Invalid start_date format".to_string()))?;

    let end_date = query
        .end_date
        .map(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d"))
        .transpose()
        .map_err(|_| Error::BadRequest("Invalid end_date format".to_string()))?;

    let min_amount: Option<Decimal> = query
        .min_amount
        .map(|a| a.parse())
        .transpose()
        .map_err(|_| Error::BadRequest("Invalid min_amount".to_string()))?;

    let max_amount: Option<Decimal> = query
        .max_amount
        .map(|a| a.parse())
        .transpose()
        .map_err(|_| Error::BadRequest("Invalid max_amount".to_string()))?;

    let page = query.page.unwrap_or(0);
    let per_page = query.per_page.unwrap_or(50);

    let filters = TransactionFilters {
        account_id: query.account_id,
        category_id: query.category_id,
        start_date,
        end_date,
        transaction_type: query.transaction_type,
        search: query.search,
        min_amount,
        max_amount,
        page: Some(page),
        per_page: Some(per_page),
    };

    let txns = transactions::Model::find_by_user(&ctx.db, user.id, &filters).await?;

    let response = TransactionsListResponse {
        transactions: txns.into_iter().map(TransactionResponse::from).collect(),
        page,
        per_page,
    };

    format::json(response)
}

/// Get recent transactions (for dashboard)
#[debug_handler]
async fn recent_transactions(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let txns = transactions::Model::find_recent(&ctx.db, user.id, 10).await?;

    let response: Vec<TransactionResponse> =
        txns.into_iter().map(TransactionResponse::from).collect();

    format::json(response)
}

/// Get a single transaction by pid
#[debug_handler]
async fn get_transaction(
    auth: auth::JWT,
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let txn = transactions::Model::find_by_pid(&ctx.db, &pid).await?;

    // Check ownership
    if txn.user_id != user.id {
        return Err(Error::NotFound);
    }

    format::json(TransactionResponse::from(txn))
}

/// Create a new transaction
#[debug_handler]
async fn create_transaction(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(req): Json<CreateTransactionRequest>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // Verify account ownership
    let account = bank_accounts::Model::find_by_user(&ctx.db, user.id)
        .await?
        .into_iter()
        .find(|a| a.id == req.account_id)
        .ok_or_else(|| Error::BadRequest("Account not found".to_string()))?;

    let transaction_date = NaiveDate::parse_from_str(&req.transaction_date, "%Y-%m-%d")
        .map_err(|_| Error::BadRequest("Invalid transaction_date format".to_string()))?;

    let posted_date = req
        .posted_date
        .map(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d"))
        .transpose()
        .map_err(|_| Error::BadRequest("Invalid posted_date format".to_string()))?;

    let amount: Decimal = req
        .amount
        .parse()
        .map_err(|_| Error::BadRequest("Invalid amount".to_string()))?;

    let params = CreateTransactionParams {
        account_id: account.id,
        category_id: req.category_id,
        statement_id: None,
        transaction_date,
        posted_date,
        description: req.description,
        original_description: req.original_description,
        amount,
        transaction_type: req.transaction_type,
        merchant_name: req.merchant_name,
        reference_number: req.reference_number,
        notes: req.notes,
    };

    let txn = transactions::Model::create(&ctx.db, user.id, &params).await?;

    format::json(TransactionResponse::from(txn))
}

/// Update a transaction
#[debug_handler]
async fn update_transaction(
    auth: auth::JWT,
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    Json(req): Json<UpdateTransactionRequest>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let txn = transactions::Model::find_by_pid(&ctx.db, &pid).await?;

    // Check ownership
    if txn.user_id != user.id {
        return Err(Error::NotFound);
    }

    let params = UpdateTransactionParams {
        category_id: req.category_id,
        description: req.description,
        merchant_name: req.merchant_name,
        notes: req.notes,
        is_recurring: req.is_recurring,
        is_excluded: req.is_excluded,
    };

    let updated =
        transactions::Model::update_transaction(&ctx.db, txn.id, user.id, &params).await?;

    format::json(TransactionResponse::from(updated))
}

/// Delete a transaction
#[debug_handler]
async fn delete_transaction(
    auth: auth::JWT,
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let txn = transactions::Model::find_by_pid(&ctx.db, &pid).await?;

    // Check ownership
    if txn.user_id != user.id {
        return Err(Error::NotFound);
    }

    use crate::models::_entities::transactions as txn_entity;
    txn_entity::Entity::delete_by_id(txn.id).exec(&ctx.db).await?;

    format::json(serde_json::json!({"status": "deleted"}))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/v1/transactions")
        .add("/", get(list_transactions))
        .add("/", post(create_transaction))
        .add("/recent", get(recent_transactions))
        .add("/{pid}", get(get_transaction))
        .add("/{pid}", put(update_transaction))
        .add("/{pid}", delete(delete_transaction))
}
