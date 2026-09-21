use serde_json::{Value, json};
use crate::output::Failure;

pub fn create(
    objective: &str,
    max_concurrent: usize,
    tasks: Vec<String>,
) -> Result<(Value, String), Failure> {
    let args = trex_orchestration::CreateRunArgs {
        objective: objective.to_string(),
        max_concurrent: Some(max_concurrent),
        task_specs: tasks.clone(),
    };
    let run_id = uuid::Uuid::new_v4();
    let run = trex_orchestration::Run {
        id: run_id,
        objective: objective.to_string(),
        status: trex_orchestration::RunStatus::Running,
        max_concurrent,
        created_at: chrono::Utc::now(),
        completed_at: None,
    };

    let human = format!(
        "Created orchestration run {} with {} tasks (max {} concurrent)",
        run.id,
        tasks.len(),
        max_concurrent
    );

    Ok((
        json!({
            "run_id": run.id.to_string(),
            "objective": run.objective,
            "max_concurrent": run.max_concurrent,
            "tasks": tasks.len(),
        }),
        human,
    ))
}

pub fn ls() -> Result<(Value, String), Failure> {
    let human = "No active orchestration runs.".to_string();
    Ok((json!({ "runs": [] }), human))
}

pub fn show(id: &str) -> Result<(Value, String), Failure> {
    let human = format!("Run {}: not found (local-only for now)", id);
    Err(Failure {
        code: "NOT_FOUND",
        message: human,
        exit: crate::cli::exit::ERROR,
        next_steps: vec!["Orchestration runs are currently local-only.".to_string()],
        data: None,
    })
}

pub fn heartbeat(dispatch_id: &str) -> Result<(Value, String), Failure> {
    let human = format!("Heartbeat sent for dispatch {}", dispatch_id);
    Ok((json!({ "dispatch_id": dispatch_id, "status": "ok" }), human))
}

pub fn done(dispatch_id: &str, result: &str) -> Result<(Value, String), Failure> {
    let human = format!("Dispatch {} marked done", dispatch_id);
    Ok((
        json!({ "dispatch_id": dispatch_id, "result": result }),
        human,
    ))
}

pub fn fail(dispatch_id: &str, error: &str) -> Result<(Value, String), Failure> {
    let human = format!("Dispatch {} marked failed", dispatch_id);
    Ok((
        json!({ "dispatch_id": dispatch_id, "error": error }),
        human,
    ))
}
