use chrono::{Duration, Utc};
use trex_accounts::service::AccountService;
use trex_accounts::{AccountProvider, AddAccountArgs, RateLimitState, UsageSnapshot};

fn add(svc: &mut AccountService, provider: AccountProvider, email: &str) -> trex_accounts::ManagedAccount {
    svc.add(AddAccountArgs {
        provider,
        email: Some(email.to_string()),
        api_key: None,
    })
}

#[test]
fn first_account_is_active_and_later_ones_are_not() {
    let mut svc = AccountService::new();
    let a = add(&mut svc, AccountProvider::Claude, "a@x.io");
    let b = add(&mut svc, AccountProvider::Codex, "b@x.io");
    assert!(a.is_active);
    assert!(!b.is_active);
    assert_eq!(svc.list().len(), 2);
    assert_eq!(svc.active().unwrap().id, a.id);
}

#[test]
fn switch_to_moves_activation() {
    let mut svc = AccountService::new();
    let a = add(&mut svc, AccountProvider::Claude, "a@x.io");
    let b = add(&mut svc, AccountProvider::Claude, "b@x.io");
    assert!(svc.switch_to(&b.id));
    assert_eq!(svc.active().unwrap().id, b.id);
    assert!(svc.active().unwrap().is_active);
    assert!(!svc.switch_to("nope"));
    assert_eq!(svc.active().unwrap().id, b.id);
}

#[test]
fn removing_active_account_activates_the_next() {
    let mut svc = AccountService::new();
    let a = add(&mut svc, AccountProvider::Claude, "a@x.io");
    let b = add(&mut svc, AccountProvider::Claude, "b@x.io");
    svc.remove(&a.id);
    assert_eq!(svc.list().len(), 1);
    assert_eq!(svc.active().unwrap().id, b.id);
    assert!(svc.active().unwrap().is_active);
}

#[test]
fn removing_the_only_account_leaves_none_active() {
    let mut svc = AccountService::new();
    let a = add(&mut svc, AccountProvider::Claude, "a@x.io");
    svc.remove(&a.id);
    assert!(svc.list().is_empty());
    assert!(svc.active().is_none());
}

#[test]
fn switch_to_unknown_id_is_a_no_op() {
    let mut svc = AccountService::new();
    add(&mut svc, AccountProvider::Claude, "a@x.io");
    assert!(!svc.switch_to("nope"));
    assert_eq!(svc.list().len(), 1);
}

#[test]
fn rate_limits_and_usage_are_tracked_per_account() {
    let mut svc = AccountService::new();
    let a = add(&mut svc, AccountProvider::Claude, "a@x.io");
    let exhausted = RateLimitState {
        provider: AccountProvider::Claude,
        account_id: a.id.clone(),
        limit: Some(100),
        remaining: Some(0),
        reset_at: Some(Utc::now() + Duration::minutes(5)),
    };
    svc.update_rate_limit(exhausted.clone());
    let got = svc.get_rate_limit(&AccountProvider::Claude, &a.id).unwrap();
    assert!(got.is_exhausted());
    assert!(got.reset_countdown().is_some());

    let usage = UsageSnapshot {
        provider: AccountProvider::Claude,
        account_id: a.id.clone(),
        total_input_tokens: 1000,
        total_output_tokens: 500,
        total_cost_usd: 0.01,
        period_start: Utc::now() - Duration::hours(1),
        period_end: Utc::now(),
    };
    svc.update_usage(usage.clone());
    let got = svc.get_usage(&AccountProvider::Claude, &a.id).unwrap();
    assert_eq!(got.total_input_tokens, 1000);

    assert!(svc.get_rate_limit(&AccountProvider::Codex, &a.id).is_none());
}