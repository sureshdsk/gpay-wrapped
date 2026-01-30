use chrono::NaiveDate;
use loco_rs::prelude::*;
use rust_decimal::Decimal;
use sea_orm::{ActiveValue, Condition, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub use super::_entities::transactions::{self, ActiveModel, Entity, Model};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTransactionParams {
    pub account_id: i32,
    pub category_id: Option<i32>,
    pub statement_id: Option<i32>,
    pub transaction_date: NaiveDate,
    pub posted_date: Option<NaiveDate>,
    pub description: String,
    pub original_description: Option<String>,
    pub amount: Decimal,
    pub transaction_type: String, // "debit" or "credit"
    pub merchant_name: Option<String>,
    pub reference_number: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTransactionParams {
    pub category_id: Option<i32>,
    pub description: Option<String>,
    pub merchant_name: Option<String>,
    pub notes: Option<String>,
    pub is_recurring: Option<bool>,
    pub is_excluded: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TransactionFilters {
    pub account_id: Option<i32>,
    pub category_id: Option<i32>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub transaction_type: Option<String>,
    pub search: Option<String>,
    pub min_amount: Option<Decimal>,
    pub max_amount: Option<Decimal>,
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for super::_entities::transactions::ActiveModel {
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
    /// Find all transactions for a user with optional filters
    pub async fn find_by_user(
        db: &DatabaseConnection,
        user_id: i32,
        filters: &TransactionFilters,
    ) -> ModelResult<Vec<Self>> {
        let mut condition = Condition::all().add(transactions::Column::UserId.eq(user_id));

        if let Some(account_id) = filters.account_id {
            condition = condition.add(transactions::Column::AccountId.eq(account_id));
        }
        if let Some(category_id) = filters.category_id {
            condition = condition.add(transactions::Column::CategoryId.eq(category_id));
        }
        if let Some(start_date) = filters.start_date {
            condition = condition.add(transactions::Column::TransactionDate.gte(start_date));
        }
        if let Some(end_date) = filters.end_date {
            condition = condition.add(transactions::Column::TransactionDate.lte(end_date));
        }
        if let Some(ref transaction_type) = filters.transaction_type {
            condition = condition.add(transactions::Column::TransactionType.eq(transaction_type.clone()));
        }
        if let Some(ref search) = filters.search {
            condition = condition.add(
                Condition::any()
                    .add(transactions::Column::Description.contains(search))
                    .add(transactions::Column::MerchantName.contains(search)),
            );
        }
        if let Some(min_amount) = filters.min_amount {
            condition = condition.add(transactions::Column::Amount.gte(min_amount));
        }
        if let Some(max_amount) = filters.max_amount {
            condition = condition.add(transactions::Column::Amount.lte(max_amount));
        }

        let page = filters.page.unwrap_or(0);
        let per_page = filters.per_page.unwrap_or(50);

        let txns = transactions::Entity::find()
            .filter(condition)
            .order_by_desc(transactions::Column::TransactionDate)
            .offset(page * per_page)
            .limit(per_page)
            .all(db)
            .await?;
        Ok(txns)
    }

    /// Find a transaction by pid
    pub async fn find_by_pid(db: &DatabaseConnection, pid: &str) -> ModelResult<Self> {
        let parse_uuid = Uuid::parse_str(pid).map_err(|e| ModelError::Any(e.into()))?;
        let txn = transactions::Entity::find()
            .filter(
                model::query::condition()
                    .eq(transactions::Column::Pid, parse_uuid)
                    .build(),
            )
            .one(db)
            .await?;
        txn.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Create a new transaction
    pub async fn create(
        db: &DatabaseConnection,
        user_id: i32,
        params: &CreateTransactionParams,
    ) -> ModelResult<Self> {
        let active = ActiveModel {
            user_id: ActiveValue::Set(user_id),
            account_id: ActiveValue::Set(params.account_id),
            category_id: ActiveValue::Set(params.category_id),
            statement_id: ActiveValue::Set(params.statement_id),
            transaction_date: ActiveValue::Set(params.transaction_date),
            posted_date: ActiveValue::Set(params.posted_date),
            description: ActiveValue::Set(params.description.clone()),
            original_description: ActiveValue::Set(params.original_description.clone()),
            amount: ActiveValue::Set(params.amount),
            transaction_type: ActiveValue::Set(params.transaction_type.clone()),
            status: ActiveValue::Set("posted".to_string()),
            merchant_name: ActiveValue::Set(params.merchant_name.clone()),
            reference_number: ActiveValue::Set(params.reference_number.clone()),
            notes: ActiveValue::Set(params.notes.clone()),
            is_recurring: ActiveValue::Set(false),
            is_excluded: ActiveValue::Set(false),
            ..Default::default()
        };
        active.insert(db).await.map_err(ModelError::from)
    }

    /// Update a transaction
    pub async fn update_transaction(
        db: &DatabaseConnection,
        id: i32,
        user_id: i32,
        params: &UpdateTransactionParams,
    ) -> ModelResult<Self> {
        let txn = transactions::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;

        // Check ownership
        if txn.user_id != user_id {
            return Err(ModelError::msg("Transaction not found"));
        }

        let mut active: ActiveModel = txn.into();
        if params.category_id.is_some() {
            active.category_id = ActiveValue::Set(params.category_id);
        }
        if let Some(description) = &params.description {
            active.description = ActiveValue::Set(description.clone());
        }
        if params.merchant_name.is_some() {
            active.merchant_name = ActiveValue::Set(params.merchant_name.clone());
        }
        if params.notes.is_some() {
            active.notes = ActiveValue::Set(params.notes.clone());
        }
        if let Some(is_recurring) = params.is_recurring {
            active.is_recurring = ActiveValue::Set(is_recurring);
        }
        if let Some(is_excluded) = params.is_excluded {
            active.is_excluded = ActiveValue::Set(is_excluded);
        }

        active.update(db).await.map_err(ModelError::from)
    }

    /// Get recent transactions for a user (for dashboard)
    pub async fn find_recent(
        db: &DatabaseConnection,
        user_id: i32,
        limit: u64,
    ) -> ModelResult<Vec<Self>> {
        let txns = transactions::Entity::find()
            .filter(transactions::Column::UserId.eq(user_id))
            .order_by_desc(transactions::Column::TransactionDate)
            .limit(limit)
            .all(db)
            .await?;
        Ok(txns)
    }

    /// Get spending summary by category for a date range
    pub async fn get_spending_by_category(
        db: &DatabaseConnection,
        user_id: i32,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> ModelResult<Vec<(Option<i32>, Decimal)>> {
        use sea_orm::{FromQueryResult, QuerySelect};

        #[derive(Debug, FromQueryResult)]
        struct CategorySum {
            category_id: Option<i32>,
            total: Decimal,
        }

        let results: Vec<CategorySum> = transactions::Entity::find()
            .filter(
                Condition::all()
                    .add(transactions::Column::UserId.eq(user_id))
                    .add(transactions::Column::TransactionDate.gte(start_date))
                    .add(transactions::Column::TransactionDate.lte(end_date))
                    .add(transactions::Column::TransactionType.eq("debit"))
                    .add(transactions::Column::IsExcluded.eq(false)),
            )
            .select_only()
            .column(transactions::Column::CategoryId)
            .column_as(transactions::Column::Amount.sum(), "total")
            .group_by(transactions::Column::CategoryId)
            .into_model::<CategorySum>()
            .all(db)
            .await?;

        Ok(results.into_iter().map(|r| (r.category_id, r.total)).collect())
    }

    /// Generate a hash for deduplication based on key transaction fields
    pub fn generate_hash(
        user_id: i32,
        account_id: i32,
        transaction_date: NaiveDate,
        amount: Decimal,
        description: &str,
        transaction_type: &str,
        reference_number: Option<&str>,
    ) -> String {
        let mut hasher = DefaultHasher::new();

        // Normalize description for hashing (trim, lowercase)
        let normalized_description = description.trim().to_lowercase();

        // Hash the key fields
        user_id.hash(&mut hasher);
        account_id.hash(&mut hasher);
        transaction_date.hash(&mut hasher);
        // Normalize amount for hashing (use absolute value since +/- is in transaction_type)
        amount.abs().normalize().to_string().hash(&mut hasher);
        normalized_description.hash(&mut hasher);
        transaction_type.hash(&mut hasher);
        // Include reference number if present (UPI IDs, check numbers, etc.)
        if let Some(ref_no) = reference_number {
            ref_no.trim().to_lowercase().hash(&mut hasher);
        }

        format!("{:x}", hasher.finish())
    }

    /// Find a duplicate transaction by reference number (bank transaction ID)
    /// This is the most reliable way to detect duplicates across accounts
    pub async fn find_duplicate_by_reference(
        db: &DatabaseConnection,
        reference_number: &str,
    ) -> ModelResult<Option<Self>> {
        if reference_number.trim().is_empty() {
            return Ok(None);
        }

        let normalized_ref = reference_number.trim().to_lowercase();

        let duplicate = transactions::Entity::find()
            .filter(
                Condition::all()
                    .add(transactions::Column::ReferenceNumber.eq(normalized_ref)),
            )
            .one(db)
            .await?;

        Ok(duplicate)
    }

    /// Find a duplicate transaction by hash
    pub async fn find_duplicate_by_hash(
        db: &DatabaseConnection,
        user_id: i32,
        account_id: i32,
        transaction_date: NaiveDate,
        amount: Decimal,
        description: &str,
        transaction_type: &str,
        reference_number: Option<&str>,
    ) -> ModelResult<Option<Self>> {
        let hash = Self::generate_hash(user_id, account_id, transaction_date, amount, description, transaction_type, reference_number);

        let duplicate = transactions::Entity::find()
            .filter(
                Condition::all()
                    .add(transactions::Column::UserId.eq(user_id))
                    .add(transactions::Column::TransactionHash.eq(hash)),
            )
            .one(db)
            .await?;

        Ok(duplicate)
    }

    /// Find duplicate transactions by key fields (fallback when hash is not available)
    pub async fn find_duplicate_by_fields(
        db: &DatabaseConnection,
        user_id: i32,
        account_id: i32,
        transaction_date: NaiveDate,
        amount: Decimal,
        description: &str,
        transaction_type: &str,
    ) -> ModelResult<Option<Self>> {
        // Normalize description for comparison
        let normalized_description = description.trim().to_lowercase();

        let duplicate = transactions::Entity::find()
            .filter(
                Condition::all()
                    .add(transactions::Column::UserId.eq(user_id))
                    .add(transactions::Column::AccountId.eq(account_id))
                    .add(transactions::Column::TransactionDate.eq(transaction_date))
                    .add(transactions::Column::Amount.eq(amount.normalize()))
                    .add(transactions::Column::TransactionType.eq(transaction_type))
            )
            .all(db)
            .await?;

        // Find exact match on description (case-insensitive, trimmed)
        Ok(duplicate
            .into_iter()
            .find(|t| t.description.trim().to_lowercase() == normalized_description))
    }

    /// Create a new transaction with deduplication
    /// Returns Ok(Some(transaction)) if created, Ok(None) if duplicate found
    ///
    /// Deduplication strategy (in order of priority):
    /// 1. Check by reference_number (global uniqueness - e.g., UPI ID, check number)
    /// 2. Check by transaction_hash (user-specific - based on multiple fields)
    pub async fn create_with_deduplication(
        db: &DatabaseConnection,
        user_id: i32,
        params: &CreateTransactionParams,
    ) -> ModelResult<Option<Self>> {
        // First, check for duplicate by reference_number (most reliable)
        // Bank transaction IDs like UPI, IMPS, check numbers should be globally unique
        if let Some(ref_no) = &params.reference_number {
            if !ref_no.trim().is_empty() {
                if let Some(_) = Self::find_duplicate_by_reference(db, ref_no).await? {
                    return Ok(None);
                }
            }
        }

        // Generate hash including reference_number
        let hash = Self::generate_hash(
            user_id,
            params.account_id,
            params.transaction_date,
            params.amount,
            &params.description,
            &params.transaction_type,
            params.reference_number.as_deref(),
        );

        // Check for duplicate by hash
        let existing = transactions::Entity::find()
            .filter(
                Condition::all()
                    .add(transactions::Column::UserId.eq(user_id))
                    .add(transactions::Column::TransactionHash.eq(hash.clone())),
            )
            .one(db)
            .await?;

        if existing.is_some() {
            return Ok(None);
        }

        // No duplicate found, create the transaction
        let active = ActiveModel {
            user_id: ActiveValue::Set(user_id),
            account_id: ActiveValue::Set(params.account_id),
            category_id: ActiveValue::Set(params.category_id),
            statement_id: ActiveValue::Set(params.statement_id),
            transaction_date: ActiveValue::Set(params.transaction_date),
            posted_date: ActiveValue::Set(params.posted_date),
            description: ActiveValue::Set(params.description.clone()),
            original_description: ActiveValue::Set(params.original_description.clone()),
            amount: ActiveValue::Set(params.amount),
            transaction_type: ActiveValue::Set(params.transaction_type.clone()),
            status: ActiveValue::Set("posted".to_string()),
            merchant_name: ActiveValue::Set(params.merchant_name.clone()),
            reference_number: ActiveValue::Set(params.reference_number.clone()),
            notes: ActiveValue::Set(params.notes.clone()),
            is_recurring: ActiveValue::Set(false),
            is_excluded: ActiveValue::Set(false),
            transaction_hash: ActiveValue::Set(Some(hash)),
            ..Default::default()
        };

        Ok(Some(active.insert(db).await.map_err(ModelError::from)?))
    }

    /// Bulk import transactions with deduplication
    /// Returns (created_count, skipped_count)
    pub async fn bulk_import_with_deduplication(
        db: &DatabaseConnection,
        user_id: i32,
        transactions_list: Vec<CreateTransactionParams>,
    ) -> ModelResult<(usize, usize)> {
        let mut created_count = 0;
        let mut skipped_count = 0;

        for params in transactions_list {
            match Self::create_with_deduplication(db, user_id, &params).await? {
                Some(_) => created_count += 1,
                None => skipped_count += 1,
            }
        }

        Ok((created_count, skipped_count))
    }
}
