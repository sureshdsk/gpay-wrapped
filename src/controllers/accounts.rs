use crate::models::{
    _entities::users,
    bank_accounts::{self, CreateAccountParams, UpdateAccountParams},
};
use loco_rs::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct AccountResponse {
    pub id: i32,
    pub pid: String,
    pub name: String,
    pub account_type: String,
    pub institution: Option<String>,
    pub account_number_last4: Option<String>,
    pub currency: String,
    pub current_balance: String,
    pub available_balance: Option<String>,
    pub color: String,
    pub is_active: bool,
    pub last_synced_at: Option<String>,
    pub created_at: String,
}

impl From<bank_accounts::Model> for AccountResponse {
    fn from(account: bank_accounts::Model) -> Self {
        Self {
            id: account.id,
            pid: account.pid.to_string(),
            name: account.name,
            account_type: account.account_type,
            institution: account.institution,
            account_number_last4: account.account_number_last4,
            currency: account.currency,
            current_balance: account.current_balance.to_string(),
            available_balance: account.available_balance.map(|b| b.to_string()),
            color: account.color,
            is_active: account.is_active,
            last_synced_at: account.last_synced_at.map(|d| d.to_rfc3339()),
            created_at: account.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AccountsSummary {
    pub total_balance: String,
    pub accounts: Vec<AccountResponse>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    pub name: String,
    pub account_type: String,
    pub institution: Option<String>,
    pub account_number_last4: Option<String>,
    pub currency: Option<String>,
    pub current_balance: String,
    pub available_balance: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAccountRequest {
    pub name: Option<String>,
    pub institution: Option<String>,
    pub color: Option<String>,
    pub is_active: Option<bool>,
    pub current_balance: Option<String>,
    pub available_balance: Option<String>,
}

/// List all accounts for the current user
#[debug_handler]
async fn list_accounts(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let accounts = bank_accounts::Model::find_by_user(&ctx.db, user.id).await?;
    let total_balance = bank_accounts::Model::get_total_balance(&ctx.db, user.id).await?;

    let response = AccountsSummary {
        total_balance: total_balance.to_string(),
        accounts: accounts.into_iter().map(AccountResponse::from).collect(),
    };

    format::json(response)
}

/// Get a single account by pid
#[debug_handler]
async fn get_account(
    auth: auth::JWT,
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let account = bank_accounts::Model::find_by_pid(&ctx.db, &pid).await?;

    // Check ownership
    if account.user_id != user.id {
        return Err(Error::NotFound);
    }

    format::json(AccountResponse::from(account))
}

/// Create a new account
#[debug_handler]
async fn create_account(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(req): Json<CreateAccountRequest>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let current_balance: Decimal = req
        .current_balance
        .parse()
        .map_err(|_| Error::BadRequest("Invalid current_balance".to_string()))?;

    let available_balance: Option<Decimal> = req
        .available_balance
        .map(|b| b.parse())
        .transpose()
        .map_err(|_| Error::BadRequest("Invalid available_balance".to_string()))?;

    let params = CreateAccountParams {
        name: req.name,
        account_type: req.account_type,
        institution: req.institution,
        account_number_last4: req.account_number_last4,
        currency: req.currency,
        current_balance,
        available_balance,
        color: req.color,
    };

    let account = bank_accounts::Model::create(&ctx.db, user.id, &params).await?;

    format::json(AccountResponse::from(account))
}

/// Update an account
#[debug_handler]
async fn update_account(
    auth: auth::JWT,
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    Json(req): Json<UpdateAccountRequest>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let account = bank_accounts::Model::find_by_pid(&ctx.db, &pid).await?;

    // Check ownership
    if account.user_id != user.id {
        return Err(Error::NotFound);
    }

    let current_balance: Option<Decimal> = req
        .current_balance
        .map(|b| b.parse())
        .transpose()
        .map_err(|_| Error::BadRequest("Invalid current_balance".to_string()))?;

    let available_balance: Option<Decimal> = req
        .available_balance
        .map(|b| b.parse())
        .transpose()
        .map_err(|_| Error::BadRequest("Invalid available_balance".to_string()))?;

    let params = UpdateAccountParams {
        name: req.name,
        institution: req.institution,
        color: req.color,
        is_active: req.is_active,
        current_balance,
        available_balance,
    };

    let updated = bank_accounts::Model::update_account(&ctx.db, account.id, user.id, &params).await?;

    format::json(AccountResponse::from(updated))
}

/// Delete an account (soft delete by setting is_active = false)
#[debug_handler]
async fn delete_account(
    auth: auth::JWT,
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let account = bank_accounts::Model::find_by_pid(&ctx.db, &pid).await?;

    // Check ownership
    if account.user_id != user.id {
        return Err(Error::NotFound);
    }

    let params = UpdateAccountParams {
        name: None,
        institution: None,
        color: None,
        is_active: Some(false),
        current_balance: None,
        available_balance: None,
    };

    bank_accounts::Model::update_account(&ctx.db, account.id, user.id, &params).await?;

    format::json(serde_json::json!({"status": "deleted"}))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/v1/accounts")
        .add("/", get(list_accounts))
        .add("/", post(create_account))
        .add("/{pid}", get(get_account))
        .add("/{pid}", put(update_account))
        .add("/{pid}", delete(delete_account))
}
