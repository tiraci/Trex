use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preamble {
    pub task_id: String,
    pub dispatch_id: String,
    pub coordinator_handle: String,
    pub worker_handle: String,
    pub objective: String,
    pub cli_examples: Vec<String>,
    pub base_drift_info: Option<String>,
    pub max_depth: usize,
}

impl Preamble {
    pub fn render(&self) -> String {
        let mut lines = vec![
            format!("## Task: {}", self.task_id),
            format!("## Dispatch: {}", self.dispatch_id),
            format!("## Objective: {}", self.objective),
            String::new(),
            "### CLI Commands:".to_string(),
        ];
        for ex in &self.cli_examples {
            lines.push(format!("  {}", ex));
        }
        if let Some(drift) = &self.base_drift_info {
            lines.push(String::new());
            lines.push(format!("### Base Drift: {}", drift));
        }
        lines.push(String::new());
        lines.push("### Heartbeat: send a heartbeat every 5 minutes".to_string());
        lines.push("### Done: when complete, send worker_done".to_string());
        lines.join("\n")
    }
}
