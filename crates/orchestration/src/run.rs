use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RunStatus {
    Created,
    Running,
    Converged,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Run {
    pub id: Uuid,
    pub objective: String,
    pub status: RunStatus,
    pub max_concurrent: usize,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRunArgs {
    pub objective: String,
    pub max_concurrent: Option<usize>,
    pub task_specs: Vec<String>,
}

impl CreateRunArgs {
    pub fn max_concurrent(&self) -> usize {
        self.max_concurrent.unwrap_or(4)
    }
}
