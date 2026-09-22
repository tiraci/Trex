use chrono::Datelike;
use trex_automations::service::{AutomationEvent, AutomationService};
use trex_automations::{Automation, AutomationRunStatus, AutomationTrigger, CreateAutomationArgs};

fn create(svc: &mut AutomationService, trigger: AutomationTrigger, name: &str) -> Automation {
    svc.create(CreateAutomationArgs {
        name: name.to_string(),
        trigger,
        prompt: "Fix the issue".to_string(),
        provider: "claude".to_string(),
        workspace_path: None,
        precheck: None,
    })
}

#[test]
fn cron_next_run_is_in_the_future() {
    let next = trex_automations::schedule::cron_next_run("0 0 * * * *").unwrap();
    assert!(next > chrono::Utc::now());
}

#[test]
fn bad_cron_expression_fails() {
    assert!(trex_automations::schedule::cron_next_run("not a cron").is_err());
    assert!(trex_automations::schedule::cron_matches("not a cron", chrono::Utc::now()) == false);
}

#[test]
fn run_lifecycle_emits_events() {
    let (mut svc, mut rx) = AutomationService::new();
    let a = create(&mut svc, AutomationTrigger::Hourly, "nightly");
    let run_id = svc.start_run(a.id).unwrap();

    let started = rx.try_recv().unwrap();
    match started {
        AutomationEvent::RunStarted { automation_id, run_id: rid } => {
            assert_eq!(automation_id, a.id);
            assert_eq!(rid, run_id);
        }
        other => panic!("expected RunStarted, got {:?}", other),
    }

    svc.complete_run(run_id, "all good".to_string());
    let completed = rx.try_recv().unwrap();
    match completed {
        AutomationEvent::RunCompleted { automation_id, .. } => assert_eq!(automation_id, a.id),
        other => panic!("expected RunCompleted, got {:?}", other),
    }

    let runs = svc.runs_for(a.id);
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].status, AutomationRunStatus::Succeeded);
    assert_eq!(runs[0].output.as_deref(), Some("all good"));
    assert_eq!(svc.get(a.id).unwrap().last_run_at.is_some(), true);
}

#[test]
fn failed_run_records_error() {
    let (mut svc, mut rx) = AutomationService::new();
    let a = create(&mut svc, AutomationTrigger::Daily, "daily");
    let run_id = svc.start_run(a.id).unwrap();
    rx.try_recv().unwrap();

    svc.fail_run(run_id, "boom".to_string());
    let failed = rx.try_recv().unwrap();
    match failed {
        AutomationEvent::RunFailed { error, .. } => assert_eq!(error, "boom"),
        other => panic!("expected RunFailed, got {:?}", other),
    }
    assert_eq!(svc.runs_for(a.id)[0].status, AutomationRunStatus::Failed);
    assert_eq!(svc.runs_for(a.id)[0].error.as_deref(), Some("boom"));
}

#[test]
fn tick_does_not_trigger_future_scheduled_automations() {
    let (mut svc, _rx) = AutomationService::new();
    let a = create(&mut svc, AutomationTrigger::Hourly, "later");
    svc.tick();
    assert!(svc.runs_for(a.id).is_empty());
    assert!(a.next_run_time().unwrap() > chrono::Utc::now());
}

#[test]
fn empty_service_tick_is_a_no_op() {
    let (mut svc, _rx) = AutomationService::new();
    svc.tick();
    assert!(svc.list().is_empty());
}

#[test]
fn start_run_unknown_id_returns_none() {
    let (mut svc, _rx) = AutomationService::new();
    assert!(svc.start_run(uuid::Uuid::new_v4()).is_none());
}

#[test]
fn run_retention_is_capped() {
    let (mut svc, mut rx) = AutomationService::new();
    let a = create(&mut svc, AutomationTrigger::Weekly, "cap");
    for _ in 0..140 {
        svc.start_run(a.id).unwrap();
        rx.try_recv().unwrap();
    }
    assert_eq!(svc.runs_for(a.id).len(), 100);
}

#[test]
fn weekday_trigger_never_lands_on_a_weekend() {
    let (mut svc, _rx) = AutomationService::new();
    let a = create(&mut svc, AutomationTrigger::Weekdays, "business");
    let next = a.next_run_time().unwrap();
    assert!(!matches!(
        next.weekday(),
        chrono::Weekday::Sat | chrono::Weekday::Sun
    ));
    svc.remove(a.id);
    assert!(svc.list().is_empty());
}