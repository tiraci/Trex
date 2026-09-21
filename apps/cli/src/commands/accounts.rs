use serde_json::{Value, json};
use crate::output::Failure;
use crate::cli::AccountProviderArg;

pub fn add(
    provider: AccountProviderArg,
    email: &Option<String>,
    api_key: &Option<String>,
) -> Result<(Value, String), Failure> {
    let provider_str = match provider {
        AccountProviderArg::Claude => "claude",
        AccountProviderArg::Codex => "codex",
    };
    let human = format!("Added {} account{}", provider_str,
        email.as_deref().map(|e| format!(" ({})", e)).unwrap_or_default()
    );
    Ok((
        json!({
            "provider": provider_str,
            "email": email,
            "added": true,
        }),
        human,
    ))
}

pub fn ls() -> Result<(Value, String), Failure> {
    let human = "No accounts configured.".to_string();
    Ok((json!({ "accounts": [] }), human))
}

pub fn switch(id: &str) -> Result<(Value, String), Failure> {
    let human = format!("Switched to account {}", id);
    Ok((json!({ "account_id": id, "active": true }), human))
}

pub fn rm(id: &str) -> Result<(Value, String), Failure> {
    let human = format!("Removed account {}", id);
    Ok((json!({ "account_id": id, "removed": true }), human))
}

pub fn rate_limit() -> Result<(Value, String), Failure> {
    let human = "No rate limit data available.".to_string();
    Ok((json!({ "rate_limit": null }), human))
}

pub fn usage() -> Result<(Value, String), Failure> {
    let human = "No usage data available.".to_string();
    Ok((json!({ "usage": null }), human))
}
