use chrono::NaiveDate;
use loco_rs::prelude::*;
use sea_orm::{ActiveValue, QueryOrder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use super::_entities::statements::{self, ActiveModel, Entity, Model};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatementStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

impl std::fmt::Display for StatementStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatementStatus::Pending => write!(f, "pending"),
            StatementStatus::Processing => write!(f, "processing"),
            StatementStatus::Completed => write!(f, "completed"),
            StatementStatus::Failed => write!(f, "failed"),
        }
    }
}

impl From<&str> for StatementStatus {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "processing" => StatementStatus::Processing,
            "completed" => StatementStatus::Completed,
            "failed" => StatementStatus::Failed,
            _ => StatementStatus::Pending,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateStatementParams {
    pub account_id: Option<i32>,
    pub filename: String,
    pub file_path: String,
    pub file_size: i32,
    pub file_type: String,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for super::_entities::statements::ActiveModel {
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
    /// Find all statements for a user
    pub async fn find_by_user(db: &DatabaseConnection, user_id: i32) -> ModelResult<Vec<Self>> {
        let stmts = statements::Entity::find()
            .filter(
                model::query::condition()
                    .eq(statements::Column::UserId, user_id)
                    .build(),
            )
            .order_by_desc(statements::Column::CreatedAt)
            .all(db)
            .await?;
        Ok(stmts)
    }

    /// Find a statement by pid
    pub async fn find_by_pid(db: &DatabaseConnection, pid: &str) -> ModelResult<Self> {
        let parse_uuid = Uuid::parse_str(pid).map_err(|e| ModelError::Any(e.into()))?;
        let stmt = statements::Entity::find()
            .filter(
                model::query::condition()
                    .eq(statements::Column::Pid, parse_uuid)
                    .build(),
            )
            .one(db)
            .await?;
        stmt.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Create a new statement
    pub async fn create(
        db: &DatabaseConnection,
        user_id: i32,
        params: &CreateStatementParams,
    ) -> ModelResult<Self> {
        let active = ActiveModel {
            user_id: ActiveValue::Set(user_id),
            account_id: ActiveValue::Set(params.account_id),
            filename: ActiveValue::Set(params.filename.clone()),
            file_path: ActiveValue::Set(params.file_path.clone()),
            file_size: ActiveValue::Set(params.file_size),
            file_type: ActiveValue::Set(params.file_type.clone()),
            status: ActiveValue::Set(StatementStatus::Pending.to_string()),
            transaction_count: ActiveValue::Set(0),
            ..Default::default()
        };
        active.insert(db).await.map_err(ModelError::from)
    }

    /// Update statement status
    pub async fn set_status(
        self,
        db: &DatabaseConnection,
        status: StatementStatus,
        error_message: Option<String>,
    ) -> ModelResult<Self> {
        let mut active: ActiveModel = self.into();
        active.status = ActiveValue::Set(status.to_string());
        if status == StatementStatus::Failed {
            active.error_message = ActiveValue::Set(error_message);
        }
        if status == StatementStatus::Completed {
            active.processed_at = ActiveValue::Set(Some(chrono::Utc::now().into()));
        }
        active.update(db).await.map_err(ModelError::from)
    }

    /// Update statement with processing results
    pub async fn set_completed(
        self,
        db: &DatabaseConnection,
        transaction_count: i32,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
    ) -> ModelResult<Self> {
        let mut active: ActiveModel = self.into();
        active.status = ActiveValue::Set(StatementStatus::Completed.to_string());
        active.transaction_count = ActiveValue::Set(transaction_count);
        active.start_date = ActiveValue::Set(start_date);
        active.end_date = ActiveValue::Set(end_date);
        active.processed_at = ActiveValue::Set(Some(chrono::Utc::now().into()));
        active.update(db).await.map_err(ModelError::from)
    }

    /// Update statement with bank detection information
    pub async fn set_bank_info(
        self,
        db: &DatabaseConnection,
        bank_name: Option<String>,
        detection_confidence: Option<i32>,
        parser_used: Option<String>,
    ) -> ModelResult<Self> {
        let mut active: ActiveModel = self.into();
        active.bank_name = ActiveValue::Set(bank_name);
        active.detection_confidence = ActiveValue::Set(detection_confidence);
        active.parser_used = ActiveValue::Set(parser_used);
        active.update(db).await.map_err(ModelError::from)
    }
}
