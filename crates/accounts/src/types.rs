use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedAccount {
    pub id: String,
    pub provider: AccountProvider,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub api_key: Option<String>,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AccountProvider {
    Claude,
    Codex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitState {
    pub provider: AccountProvider,
    pub account_id: String,
    pub limit: Option<u64>,
    pub remaining: Option<u64>,
    pub reset_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageSnapshot {
    pub provider: AccountProvider,
    pub account_id: String,
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub total_cost_usd: f64,
    pub period_start: chrono::DateTime<chrono::Utc>,
    pub period_end: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddAccountArgs {
    pub provider: AccountProvider,
    pub email: Option<String>,
    pub api_key: Option<String>,
}

impl RateLimitState {
    pub fn is_exhausted(&self) -> bool {
        self.remaining.map_or(false, |r| r == 0)
    }

    pub fn reset_countdown(&self) -> Option<String> {
        self.reset_at.map(|reset| {
            let now = chrono::Utc::now();
            let remaining = (reset - now).num_seconds().max(0);
            let mins = remaining / 60;
            let secs = remaining % 60;
            format!("{}m {}s", mins, secs)
        })
    }
}
