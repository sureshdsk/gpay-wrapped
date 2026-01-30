use loco_rs::prelude::*;
use rust_decimal::Decimal;
use sea_orm::{ActiveValue, QueryOrder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use super::_entities::bank_accounts::{self, ActiveModel, Entity, Model};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAccountParams {
    pub name: String,
    pub account_type: String, // "checking", "savings", "credit_card", "investment"
    pub institution: Option<String>,
    pub account_number_last4: Option<String>,
    pub currency: Option<String>,
    pub current_balance: Decimal,
    pub available_balance: Option<Decimal>,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateAccountParams {
    pub name: Option<String>,
    pub institution: Option<String>,
    pub color: Option<String>,
    pub is_active: Option<bool>,
    pub current_balance: Option<Decimal>,
    pub available_balance: Option<Decimal>,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for super::_entities::bank_accounts::ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            let mut this = self;
            this.pid = ActiveValue::Set(Uuid::new_v4());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

impl Model {
    /// Find all accounts for a user
    pub async fn find_by_user(db: &DatabaseConnection, user_id: i32) -> ModelResult<Vec<Self>> {
        let accounts = bank_accounts::Entity::find()
            .filter(
                model::query::condition()
                    .eq(bank_accounts::Column::UserId, user_id)
                    .build(),
            )
            .order_by_desc(bank_accounts::Column::CreatedAt)
            .all(db)
            .await?;
        Ok(accounts)
    }

    /// Find active accounts for a user
    pub async fn find_active_by_user(db: &DatabaseConnection, user_id: i32) -> ModelResult<Vec<Self>> {
        let accounts = bank_accounts::Entity::find()
            .filter(
                model::query::condition()
                    .eq(bank_accounts::Column::UserId, user_id)
                    .eq(bank_accounts::Column::IsActive, true)
                    .build(),
            )
            .order_by_desc(bank_accounts::Column::CreatedAt)
            .all(db)
            .await?;
        Ok(accounts)
    }

    /// Find an account by pid
    pub async fn find_by_pid(db: &DatabaseConnection, pid: &str) -> ModelResult<Self> {
        let parse_uuid = Uuid::parse_str(pid).map_err(|e| ModelError::Any(e.into()))?;
        let account = bank_accounts::Entity::find()
            .filter(
                model::query::condition()
                    .eq(bank_accounts::Column::Pid, parse_uuid)
                    .build(),
            )
            .one(db)
            .await?;
        account.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Create a new bank account
    pub async fn create(
        db: &DatabaseConnection,
        user_id: i32,
        params: &CreateAccountParams,
    ) -> ModelResult<Self> {
        let active = ActiveModel {
            user_id: ActiveValue::Set(user_id),
            name: ActiveValue::Set(params.name.clone()),
            account_type: ActiveValue::Set(params.account_type.clone()),
            institution: ActiveValue::Set(params.institution.clone()),
            account_number_last4: ActiveValue::Set(params.account_number_last4.clone()),
            currency: ActiveValue::Set(params.currency.clone().unwrap_or_else(|| "USD".to_string())),
            current_balance: ActiveValue::Set(params.current_balance),
            available_balance: ActiveValue::Set(params.available_balance),
            color: ActiveValue::Set(params.color.clone().unwrap_or_else(|| "#3d84f5".to_string())),
            is_active: ActiveValue::Set(true),
            ..Default::default()
        };
        active.insert(db).await.map_err(ModelError::from)
    }

    /// Update an account
    pub async fn update_account(
        db: &DatabaseConnection,
        id: i32,
        user_id: i32,
        params: &UpdateAccountParams,
    ) -> ModelResult<Self> {
        let account = bank_accounts::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;

        // Check ownership
        if account.user_id != user_id {
            return Err(ModelError::msg("Account not found"));
        }

        let mut active: ActiveModel = account.into();
        if let Some(name) = &params.name {
            active.name = ActiveValue::Set(name.clone());
        }
        if let Some(institution) = &params.institution {
            active.institution = ActiveValue::Set(Some(institution.clone()));
        }
        if let Some(color) = &params.color {
            active.color = ActiveValue::Set(color.clone());
        }
        if let Some(is_active) = params.is_active {
            active.is_active = ActiveValue::Set(is_active);
        }
        if let Some(balance) = params.current_balance {
            active.current_balance = ActiveValue::Set(balance);
        }
        if let Some(balance) = params.available_balance {
            active.available_balance = ActiveValue::Set(Some(balance));
        }

        active.update(db).await.map_err(ModelError::from)
    }

    /// Get total balance across all active accounts for a user
    pub async fn get_total_balance(db: &DatabaseConnection, user_id: i32) -> ModelResult<Decimal> {
        let accounts = Self::find_active_by_user(db, user_id).await?;
        let total = accounts.iter().map(|a| a.current_balance).sum();
        Ok(total)
    }
}
