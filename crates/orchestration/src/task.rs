use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    Ready,
    Dispatched,
    Completed,
    Failed,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub run_id: Uuid,
    pub spec: String,
    pub status: TaskStatus,
    pub deps: Vec<Uuid>,
    pub parent_id: Option<Uuid>,
    pub result: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskGraph {
    pub run_id: Uuid,
    pub tasks: Vec<Task>,
}

impl TaskGraph {
    pub fn ready_tasks(&self) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Ready)
            .collect()
    }

    pub fn is_converged(&self) -> bool {
        self.tasks.iter().all(|t| {
            matches!(
                t.status,
                TaskStatus::Completed | TaskStatus::Failed | TaskStatus::Blocked
            )
        })
    }

    pub fn has_active(&self) -> bool {
        self.tasks
            .iter()
            .any(|t| t.status == TaskStatus::Dispatched)
    }
}
