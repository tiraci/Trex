use crate::types::{Automation, AutomationRun, AutomationRunStatus, CreateAutomationArgs};
use anyhow::Result;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tracing::{info, warn};
use uuid::Uuid;

pub struct AutomationService {
    automations: HashMap<Uuid, Automation>,
    runs: Vec<AutomationRun>,
    event_tx: mpsc::UnboundedSender<AutomationEvent>,
}

#[derive(Debug, Clone)]
pub enum AutomationEvent {
    RunStarted { automation_id: Uuid, run_id: Uuid },
    RunCompleted { automation_id: Uuid, run_id: Uuid },
    RunFailed { automation_id: Uuid, run_id: Uuid, error: String },
    RunSkipped { automation_id: Uuid, run_id: Uuid },
}

const MAX_RUNS: usize = 100;

impl AutomationService {
    pub fn new() -> (Self, mpsc::UnboundedReceiver<AutomationEvent>) {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        (
            AutomationService {
                automations: HashMap::new(),
                runs: Vec::new(),
                event_tx,
            },
            event_rx,
        )
    }

    pub fn create(&mut self, args: CreateAutomationArgs) -> Automation {
        let automation = Automation {
            id: Uuid::new_v4(),
            name: args.name,
            trigger: args.trigger,
            prompt: args.prompt,
            provider: args.provider,
            workspace_path: args.workspace_path,
            enabled: true,
            created_at: chrono::Utc::now(),
            last_run_at: None,
        };
        self.automations.insert(automation.id, automation.clone());
        automation
    }

    pub fn tick(&mut self) {
        let now = chrono::Utc::now();
        let to_trigger: Vec<Uuid> = self
            .automations
            .values()
            .filter(|a| a.enabled)
            .filter(|a| a.next_run_time().map_or(false, |t| t <= now))
            .map(|a| {
                info!(automation = %a.name, "Triggering automation");
                a.id
            })
            .collect();
        for id in to_trigger {
            self.start_run(id);
        }
    }

    pub fn start_run(&mut self, automation_id: Uuid) -> Option<Uuid> {
        if let Some(automation) = self.automations.get_mut(&automation_id) {
            let run_id = Uuid::new_v4();
            let run = AutomationRun {
                id: run_id,
                automation_id,
                status: AutomationRunStatus::Running,
                started_at: Some(chrono::Utc::now()),
                completed_at: None,
                output: None,
                error: None,
            };
            automation.last_run_at = Some(chrono::Utc::now());
            self.runs.push(run);

            // Enforce retention
            if self.runs.len() > MAX_RUNS {
                let excess = self.runs.len() - MAX_RUNS;
                self.runs.drain(..excess);
            }

            let _ = self.event_tx.send(AutomationEvent::RunStarted {
                automation_id,
                run_id,
            });
            return Some(run_id);
        }
        None
    }

    pub fn complete_run(&mut self, run_id: Uuid, output: String) {
        if let Some(run) = self.runs.iter_mut().find(|r| r.id == run_id) {
            run.status = AutomationRunStatus::Succeeded;
            run.completed_at = Some(chrono::Utc::now());
            run.output = Some(output);
            let _ = self.event_tx.send(AutomationEvent::RunCompleted {
                automation_id: run.automation_id,
                run_id,
            });
        }
    }

    pub fn fail_run(&mut self, run_id: Uuid, error: String) {
        if let Some(run) = self.runs.iter_mut().find(|r| r.id == run_id) {
            run.status = AutomationRunStatus::Failed;
            run.completed_at = Some(chrono::Utc::now());
            run.error = Some(error.clone());
            let _ = self.event_tx.send(AutomationEvent::RunFailed {
                automation_id: run.automation_id,
                run_id,
                error,
            });
        }
    }

    pub fn list(&self) -> Vec<&Automation> {
        self.automations.values().collect()
    }

    pub fn get(&self, id: Uuid) -> Option<&Automation> {
        self.automations.get(&id)
    }

    pub fn remove(&mut self, id: Uuid) -> bool {
        self.automations.remove(&id).is_some()
    }

    pub fn runs_for(&self, automation_id: Uuid) -> Vec<&AutomationRun> {
        self.runs
            .iter()
            .filter(|r| r.automation_id == automation_id)
            .collect()
    }
}
