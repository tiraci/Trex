use chrono::Datelike;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Automation {
    pub id: Uuid,
    pub name: String,
    pub trigger: AutomationTrigger,
    pub prompt: String,
    pub provider: String,
    pub workspace_path: Option<String>,
    pub enabled: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_run_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomationTrigger {
    Cron(String),
    Hourly,
    Daily,
    Weekdays,
    Weekly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AutomationRunStatus {
    Scheduled,
    Running,
    Succeeded,
    Failed,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRun {
    pub id: Uuid,
    pub automation_id: Uuid,
    pub status: AutomationRunStatus,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub output: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAutomationArgs {
    pub name: String,
    pub trigger: AutomationTrigger,
    pub prompt: String,
    pub provider: String,
    pub workspace_path: Option<String>,
    pub precheck: Option<String>,
}

impl Automation {
    pub fn next_run_time(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        match &self.trigger {
            AutomationTrigger::Cron(expr) => crate::schedule::cron_next_run(expr).ok(),
            AutomationTrigger::Hourly => {
                let next = self
                    .last_run_at
                    .unwrap_or(chrono::Utc::now())
                    + chrono::Duration::hours(1);
                Some(next)
            }
            AutomationTrigger::Daily => {
                let now = chrono::Utc::now();
                let next = now + chrono::Duration::days(1);
                Some(next)
            }
            AutomationTrigger::Weekdays => {
                let now = chrono::Utc::now();
                let mut next = now + chrono::Duration::days(1);
                while next.weekday() == chrono::Weekday::Sat
                    || next.weekday() == chrono::Weekday::Sun
                {
                    next = next + chrono::Duration::days(1);
                }
                Some(next)
            }
            AutomationTrigger::Weekly => {
                let next = self
                    .last_run_at
                    .unwrap_or(chrono::Utc::now())
                    + chrono::Duration::weeks(1);
                Some(next)
            }
        }
    }
}
