use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConvergenceState {
    AllDone,
    Active,
    Empty,
    Stuck,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagConvergence {
    pub state: ConvergenceState,
    pub total_tasks: usize,
    pub completed: usize,
    pub failed: usize,
    pub active: usize,
    pub blocked: usize,
}

impl DagConvergence {
    pub fn evaluate(tasks: &[super::task::Task]) -> Self {
        let total = tasks.len();
        let completed = tasks
            .iter()
            .filter(|t| t.status == super::task::TaskStatus::Completed)
            .count();
        let failed = tasks
            .iter()
            .filter(|t| t.status == super::task::TaskStatus::Failed)
            .count();
        let active = tasks
            .iter()
            .filter(|t| t.status == super::task::TaskStatus::Dispatched)
            .count();
        let blocked = tasks
            .iter()
            .filter(|t| t.status == super::task::TaskStatus::Blocked)
            .count();

        let state = if total == 0 {
            ConvergenceState::Empty
        } else if completed + failed == total {
            ConvergenceState::AllDone
        } else if active > 0 {
            ConvergenceState::Active
        } else if blocked > 0 {
            ConvergenceState::Stuck
        } else {
            ConvergenceState::Active
        };

        DagConvergence {
            state,
            total_tasks: total,
            completed,
            failed,
            active,
            blocked,
        }
    }
}
