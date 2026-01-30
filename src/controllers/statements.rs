use crate::models::{
    _entities::users,
    statements::{self, CreateStatementParams, StatementStatus},
    transactions::{CreateTransactionParams, Model as TransactionModel},
};
use crate::parsers::{ParserOptions, ParserRegistry, ParsedTransaction, TransactionType};
use axum_extra::extract::Multipart;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const UPLOAD_DIR: &str = "uploads/statements";
const MAX_FILE_SIZE: usize = 50 * 1024 * 1024; // 50MB

#[derive(Debug, Serialize)]
pub struct StatementResponse {
    pub id: i32,
    pub pid: String,
    pub account_id: Option<i32>,
    pub filename: String,
    pub file_type: String,
    pub file_size: i32,
    pub status: String,
    pub statement_date: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub transaction_count: i32,
    pub error_message: Option<String>,
    pub processed_at: Option<String>,
    pub created_at: String,
    pub bank_name: Option<String>,
    pub detection_confidence: Option<i32>,
    pub parser_used: Option<String>,
}

impl From<statements::Model> for StatementResponse {
    fn from(stmt: statements::Model) -> Self {
        Self {
            id: stmt.id,
            pid: stmt.pid.to_string(),
            account_id: stmt.account_id,
            filename: stmt.filename,
            file_type: stmt.file_type,
            file_size: stmt.file_size,
            status: stmt.status,
            statement_date: stmt.statement_date.map(|d| d.to_string()),
            start_date: stmt.start_date.map(|d| d.to_string()),
            end_date: stmt.end_date.map(|d| d.to_string()),
            transaction_count: stmt.transaction_count,
            error_message: stmt.error_message,
            processed_at: stmt.processed_at.map(|d| d.to_rfc3339()),
            created_at: stmt.created_at.to_rfc3339(),
            bank_name: stmt.bank_name,
            detection_confidence: stmt.detection_confidence,
            parser_used: stmt.parser_used,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UploadResponse {
    pub statement: StatementResponse,
    pub preview: Vec<TransactionPreview>,
}

#[derive(Debug, Serialize)]
pub struct TransactionPreview {
    pub date: String,
    pub description: String,
    pub amount: String,
    pub transaction_type: String,
    pub balance: Option<String>,
}

impl From<ParsedTransaction> for TransactionPreview {
    fn from(tx: ParsedTransaction) -> Self {
        Self {
            date: tx.date.to_string(),
            description: tx.description,
            amount: tx.amount.to_string(),
            transaction_type: tx.transaction_type.to_string(),
            balance: tx.balance.map(|b| b.to_string()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ConfirmImportRequest {
    pub account_id: i32,
}

/// List all statements for the current user
#[debug_handler]
async fn list_statements(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let stmts = statements::Model::find_by_user(&ctx.db, user.id).await?;
    let response: Vec<StatementResponse> = stmts.into_iter().map(StatementResponse::from).collect();
    format::json(response)
}

/// Get a single statement by pid
#[debug_handler]
async fn get_statement(
    auth: auth::JWT,
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let stmt = statements::Model::find_by_pid(&ctx.db, &pid).await?;

    if stmt.user_id != user.id {
        return Err(Error::NotFound);
    }

    format::json(StatementResponse::from(stmt))
}

/// Upload a statement file
/// Accepts multipart form data with:
/// - file: The statement file (Excel .xls or .xlsx)
/// - account_id: Optional account ID to associate with
#[debug_handler]
async fn upload_statement(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    mut multipart: Multipart,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let mut file_data: Option<Vec<u8>> = None;
    let mut filename: Option<String> = None;
    let mut account_id: Option<i32> = None;

    // Process multipart fields
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        tracing::error!(error = %e, "Failed to read multipart field");
        Error::BadRequest("Failed to read upload data".to_string())
    })? {
        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "file" => {
                filename = field.file_name().map(|s| s.to_string());
                let data = field.bytes().await.map_err(|e| {
                    tracing::error!(error = %e, "Failed to read file data");
                    Error::BadRequest("Failed to read file data".to_string())
                })?;

                if data.len() > MAX_FILE_SIZE {
                    return Err(Error::BadRequest(format!(
                        "File too large. Maximum size is {} MB",
                        MAX_FILE_SIZE / 1024 / 1024
                    )));
                }

                file_data = Some(data.to_vec());
            }
            "account_id" => {
                let text = field.text().await.unwrap_or_default();
                account_id = text.parse().ok();
            }
            _ => {}
        }
    }

    // Validate required fields
    let file_data = file_data.ok_or_else(|| Error::BadRequest("No file uploaded".to_string()))?;
    let filename = filename.ok_or_else(|| Error::BadRequest("No filename provided".to_string()))?;

    // Determine file type from extension
    let file_ext = filename
        .rsplit('.')
        .next()
        .unwrap_or("")
        .to_lowercase();

    let file_type = match file_ext.as_str() {
        "xlsx" | "xls" => "excel",
        _ => return Err(Error::BadRequest(format!("Unsupported file type: {}. Only Excel files (.xls, .xlsx) are supported.", file_ext))),
    };

    // Create upload directory if it doesn't exist
    let upload_path = PathBuf::from(UPLOAD_DIR);
    std::fs::create_dir_all(&upload_path).map_err(|e| {
        tracing::error!(error = %e, "Failed to create upload directory");
        Error::InternalServerError
    })?;

    // Generate unique filename
    let unique_filename = format!("{}_{}", uuid::Uuid::new_v4(), filename);
    let file_path = upload_path.join(&unique_filename);

    // Write file to disk
    std::fs::write(&file_path, &file_data).map_err(|e| {
        tracing::error!(error = %e, path = ?file_path, "Failed to write file");
        Error::InternalServerError
    })?;

    // Create statement record
    let params = CreateStatementParams {
        account_id,
        filename: filename.clone(),
        file_path: file_path.to_string_lossy().to_string(),
        file_size: file_data.len() as i32,
        file_type: file_type.to_string(),
    };

    let stmt = statements::Model::create(&ctx.db, user.id, &params).await?;

    // Parse the file using auto-detection
    let registry = ParserRegistry::new();
    let parser_options = ParserOptions::default();

    // Use auto_parse to detect bank and parse
    let parse_result = registry.auto_parse(&filename, &file_data, &parser_options);

    match parse_result {
        Ok(result) => {
            // Get detection info for bank storage
            let detection = registry.detector().detect(&filename, &file_data).ok();
            let bank_name = result.bank_name.clone();
            let detection_confidence = detection.as_ref().map(|d| (d.confidence * 100.0) as i32);
            let parser_used = detection.as_ref().map(|d| d.suggested_parser.clone());

            // Update statement with parsing info
            let stmt = stmt
                .set_completed(
                    &ctx.db,
                    result.transactions.len() as i32,
                    result.start_date,
                    result.end_date,
                )
                .await?;

            // Store bank detection information
            let stmt = stmt
                .set_bank_info(
                    &ctx.db,
                    bank_name,
                    detection_confidence,
                    parser_used,
                )
                .await?;

            // Store parsed transactions in session/cache for later confirmation
            // For now, return preview
            let preview: Vec<TransactionPreview> = result
                .transactions
                .into_iter()
                .map(TransactionPreview::from)
                .collect();

            format::json(UploadResponse {
                statement: StatementResponse::from(stmt),
                preview,
            })
        }
        Err(e) => {
            // Update statement with error
            let error_msg = e.to_string();
            let _stmt = stmt
                .set_status(&ctx.db, StatementStatus::Failed, Some(error_msg.clone()))
                .await?;

            Err(Error::BadRequest(format!("Failed to parse file: {}", error_msg)))
        }
    }
}

/// Confirm import of parsed transactions
#[debug_handler]
async fn confirm_import(
    auth: auth::JWT,
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    Json(req): Json<ConfirmImportRequest>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let stmt = statements::Model::find_by_pid(&ctx.db, &pid).await?;

    if stmt.user_id != user.id {
        return Err(Error::NotFound);
    }

    if stmt.status != "completed" {
        return Err(Error::BadRequest("Statement is not ready for import".to_string()));
    }

    // Re-parse the file to get transactions
    let file_data = std::fs::read(&stmt.file_path).map_err(|e| {
        tracing::error!(error = %e, path = stmt.file_path, "Failed to read statement file");
        Error::InternalServerError
    })?;

    let registry = ParserRegistry::new();
    let parser_options = ParserOptions::default();

    // Use auto_parse to detect bank and parse
    let parse_result = registry
        .auto_parse(&stmt.filename, &file_data, &parser_options)
        .map_err(|e| Error::BadRequest(format!("Failed to parse file: {}", e)))?;

    // Convert parsed transactions to CreateTransactionParams
    let transactions_params: Vec<CreateTransactionParams> = parse_result.transactions
        .into_iter()
        .map(|parsed_tx| CreateTransactionParams {
            account_id: req.account_id,
            category_id: None,
            statement_id: Some(stmt.id),
            transaction_date: parsed_tx.date,
            posted_date: None,
            description: parsed_tx.description,
            original_description: None,
            amount: parsed_tx.amount,
            transaction_type: match parsed_tx.transaction_type {
                TransactionType::Credit => "credit".to_string(),
                TransactionType::Debit => "debit".to_string(),
            },
            merchant_name: None,
            reference_number: parsed_tx.reference,
            notes: None,
        })
        .collect();

    // Import transactions with deduplication
    let (created_count, skipped_count) = TransactionModel::bulk_import_with_deduplication(
        &ctx.db,
        user.id,
        transactions_params,
    )
    .await
    .map_err(|e| Error::BadRequest(format!("Failed to import transactions: {}", e)))?;

    format::json(serde_json::json!({
        "status": "imported",
        "transactions_created": created_count,
        "transactions_skipped": skipped_count,
        "duplicates_found": skipped_count,
        "statement_id": stmt.pid.to_string(),
    }))
}

/// Delete a statement
#[debug_handler]
async fn delete_statement(
    auth: auth::JWT,
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let stmt = statements::Model::find_by_pid(&ctx.db, &pid).await?;

    if stmt.user_id != user.id {
        return Err(Error::NotFound);
    }

    // Delete the file
    if let Err(e) = std::fs::remove_file(&stmt.file_path) {
        tracing::warn!(error = %e, path = stmt.file_path, "Failed to delete statement file");
    }

    // Delete the record
    use crate::models::_entities::statements as stmt_entity;
    stmt_entity::Entity::delete_by_id(stmt.id).exec(&ctx.db).await?;

    format::json(serde_json::json!({"status": "deleted"}))
}

/// Get supported parser types
#[debug_handler]
async fn get_parsers(auth: auth::JWT, State(_ctx): State<AppContext>) -> Result<Response> {
    // Validate auth but don't need user details
    let _ = auth;

    let registry = ParserRegistry::new();
    let parsers = registry.list();
    let extensions = registry.supported_extensions();

    format::json(serde_json::json!({
        "parsers": parsers,
        "supported_extensions": extensions,
    }))
}

/// Get supported banks
#[debug_handler]
async fn get_banks(auth: auth::JWT, State(_ctx): State<AppContext>) -> Result<Response> {
    // Validate auth but don't need user details
    let _ = auth;

    let registry = ParserRegistry::new();
    let banks = registry.list_banks();

    let bank_info: Vec<serde_json::Value> = banks
        .iter()
        .filter_map(|code| registry.get_bank(code))
        .map(|bank| {
            let info = bank.info();
            serde_json::json!({
                "code": info.code,
                "name": info.name,
                "aliases": info.aliases,
            })
        })
        .collect();

    format::json(serde_json::json!({
        "banks": bank_info,
    }))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/v1/statements")
        .add("/", get(list_statements))
        .add("/upload", post(upload_statement))
        .add("/parsers", get(get_parsers))
        .add("/banks", get(get_banks))
        .add("/{pid}", get(get_statement))
        .add("/{pid}", delete(delete_statement))
        .add("/{pid}/confirm", post(confirm_import))
}
