use crate::types::AutomationTrigger;
use anyhow::Result;
use std::str::FromStr;

pub fn cron_next_run(expr: &str) -> Result<chrono::DateTime<chrono::Utc>> {
    let schedule = cron::Schedule::from_str(expr)?;
    let next = schedule
        .upcoming(chrono::Utc)
        .next()
        .ok_or_else(|| anyhow::anyhow!("No upcoming run time"))?;
    Ok(next)
}

pub fn cron_matches(expr: &str, now: chrono::DateTime<chrono::Utc>) -> bool {
    match cron::Schedule::from_str(expr) {
        Ok(schedule) => schedule
            .upcoming(chrono::Utc)
            .take(1)
            .next()
            .map(|t| t == now)
            .unwrap_or(false),
        Err(_) => false,
    }
}

pub fn parse_trigger(trigger_str: &str) -> Result<AutomationTrigger> {
    match trigger_str.to_lowercase().as_str() {
        "hourly" => Ok(AutomationTrigger::Hourly),
        "daily" => Ok(AutomationTrigger::Daily),
        "weekdays" => Ok(AutomationTrigger::Weekdays),
        "weekly" => Ok(AutomationTrigger::Weekly),
        _ => {
            // Try parsing as cron
            cron::Schedule::from_str(trigger_str)?;
            Ok(AutomationTrigger::Cron(trigger_str.to_string()))
        }
    }
}
