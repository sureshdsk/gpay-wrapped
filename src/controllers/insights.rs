use crate::models::{
    _entities::users,
    bank_accounts,
    transactions::{self, TransactionFilters},
};
use chrono::{Datelike, Utc};
use loco_rs::prelude::*;
use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DashboardSummary {
    pub total_balance: String,
    pub account_count: i64,
    pub recent_transactions: Vec<RecentTransaction>,
    pub this_month: MonthlySummary,
}

#[derive(Debug, Serialize)]
pub struct RecentTransaction {
    pub id: i32,
    pub pid: String,
    pub description: String,
    pub amount: String,
    pub transaction_type: String,
    pub transaction_date: String,
    pub category_name: Option<String>,
    pub account_name: String,
}

#[derive(Debug, Serialize)]
pub struct MonthlySummary {
    pub month: String,
    pub income: String,
    pub expenses: String,
    pub net: String,
    pub transaction_count: i64,
}

/// Get dashboard summary for the current user
#[debug_handler]
async fn summary(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // Get account balance and count
    let total_balance = bank_accounts::Model::get_total_balance(&ctx.db, user.id).await?;
    let accounts = bank_accounts::Model::find_active_by_user(&ctx.db, user.id).await?;
    let account_count = accounts.len() as i64;

    // Create account lookup
    let account_map: std::collections::HashMap<i32, String> = accounts
        .iter()
        .map(|a| (a.id, a.name.clone()))
        .collect();

    // Get recent transactions
    let recent_txns = transactions::Model::find_recent(&ctx.db, user.id, 5).await?;
    let recent_transactions: Vec<RecentTransaction> = recent_txns
        .into_iter()
        .map(|t| {
            let account_name = account_map
                .get(&t.account_id)
                .cloned()
                .unwrap_or_else(|| "Unknown Account".to_string());

            RecentTransaction {
                id: t.id,
                pid: t.pid.to_string(),
                description: t.description,
                amount: t.amount.to_string(),
                transaction_type: t.transaction_type,
                transaction_date: t.transaction_date.to_string(),
                category_name: t.category_id.map(|_| " Uncategorized".to_string()), // TODO: Join with categories
                account_name,
            }
        })
        .collect();

    // Get this month's summary
    let now = Utc::now();
    let start_of_month = chrono::NaiveDate::from_ymd_opt(now.year(), now.month(), 1)
        .ok_or_else(|| Error::BadRequest("Invalid date".to_string()))?;

    let end_of_month = if now.month() == 12 {
        chrono::NaiveDate::from_ymd_opt(now.year() + 1, 1, 1)
            .ok_or_else(|| Error::BadRequest("Invalid date".to_string()))?
    } else {
        chrono::NaiveDate::from_ymd_opt(now.year(), now.month() + 1, 1)
            .ok_or_else(|| Error::BadRequest("Invalid date".to_string()))?
    } - chrono::Days::new(1);

    // Get all transactions for this month
    let month_filters = TransactionFilters {
        account_id: None,
        category_id: None,
        start_date: Some(start_of_month),
        end_date: Some(end_of_month),
        transaction_type: None,
        search: None,
        min_amount: None,
        max_amount: None,
        page: None,
        per_page: None,
    };

    let month_txns = transactions::Model::find_by_user(&ctx.db, user.id, &month_filters).await?;

    let income: Decimal = month_txns
        .iter()
        .filter(|t| t.transaction_type == "credit" && !t.is_excluded)
        .map(|t| t.amount)
        .sum();

    let expenses: Decimal = month_txns
        .iter()
        .filter(|t| t.transaction_type == "debit" && !t.is_excluded)
        .map(|t| t.amount)
        .sum();

    let net = income - expenses;
    let transaction_count = month_txns.len() as i64;

    let this_month = MonthlySummary {
        month: format!("{} {}", now.year(), month_name(now.month())),
        income: income.to_string(),
        expenses: expenses.to_string(),
        net: net.to_string(),
        transaction_count,
    };

    let summary = DashboardSummary {
        total_balance: total_balance.to_string(),
        account_count,
        recent_transactions,
        this_month,
    };

    format::json(summary)
}

fn month_name(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Unknown",
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/v1/insights")
        .add("/summary", get(summary))
}
