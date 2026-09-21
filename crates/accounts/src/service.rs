use crate::types::{AccountProvider, AddAccountArgs, ManagedAccount, RateLimitState, UsageSnapshot};
use std::collections::HashMap;

pub struct AccountService {
    accounts: Vec<ManagedAccount>,
    rate_limits: HashMap<String, RateLimitState>,
    usage: HashMap<String, UsageSnapshot>,
}

impl AccountService {
    pub fn new() -> Self {
        AccountService {
            accounts: Vec::new(),
            rate_limits: HashMap::new(),
            usage: HashMap::new(),
        }
    }

    pub fn add(&mut self, args: AddAccountArgs) -> ManagedAccount {
        let account = ManagedAccount {
            id: uuid::Uuid::new_v4().to_string(),
            provider: args.provider,
            email: args.email,
            display_name: None,
            api_key: args.api_key,
            is_active: self.accounts.is_empty(),
            created_at: chrono::Utc::now(),
        };
        self.accounts.push(account.clone());
        account
    }

    pub fn list(&self) -> &[ManagedAccount] {
        &self.accounts
    }

    pub fn active(&self) -> Option<&ManagedAccount> {
        self.accounts.iter().find(|a| a.is_active)
    }

    pub fn switch_to(&mut self, account_id: &str) -> bool {
        let found = self.accounts.iter().any(|a| a.id == account_id);
        if found {
            for a in &mut self.accounts {
                a.is_active = a.id == account_id;
            }
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, account_id: &str) -> bool {
        let was_active = self
            .accounts
            .iter()
            .find(|a| a.id == account_id)
            .map(|a| a.is_active)
            .unwrap_or(false);
        self.accounts.retain(|a| a.id != account_id);

        // If we removed the active one, activate the first remaining
        if was_active {
            if let Some(first) = self.accounts.first_mut() {
                first.is_active = true;
            }
        }
        true
    }

    pub fn update_rate_limit(&mut self, state: RateLimitState) {
        let key = format!("{}:{}", state.provider as u8, state.account_id);
        self.rate_limits.insert(key, state);
    }

    pub fn get_rate_limit(&self, provider: &AccountProvider, account_id: &str) -> Option<&RateLimitState> {
        let key = format!("{}:{}", *provider as u8, account_id);
        self.rate_limits.get(&key)
    }

    pub fn update_usage(&mut self, snapshot: UsageSnapshot) {
        let key = format!("{}:{}", snapshot.provider as u8, snapshot.account_id);
        self.usage.insert(key, snapshot);
    }

    pub fn get_usage(&self, provider: &AccountProvider, account_id: &str) -> Option<&UsageSnapshot> {
        let key = format!("{}:{}", *provider as u8, account_id);
        self.usage.get(&key)
    }
}
