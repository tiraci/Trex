use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DispatchState {
    Starting,
    Ready,
    Running,
    Succeeded,
    Failed,
    Stopped,
    Abandoned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerDispatch {
    pub dispatch_id: Uuid,
    pub task_id: Uuid,
    pub worker_id: String,
    pub worktree_path: Option<String>,
    pub state: DispatchState,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_heartbeat: Option<chrono::DateTime<chrono::Utc>>,
}

impl WorkerDispatch {
    pub fn is_alive(&self) -> bool {
        matches!(
            self.state,
            DispatchState::Starting | DispatchState::Ready | DispatchState::Running
        )
    }

    pub fn is_stale(&self) -> bool {
        let now = chrono::Utc::now();
        match self.last_heartbeat {
            Some(ts) => (now - ts).num_minutes() >= 10,
            None => (now - self.created_at).num_minutes() >= 10,
        }
    }
}
