use crate::dag::{ConvergenceState, DagConvergence};
use crate::dispatch::{DispatchState, WorkerDispatch};
use crate::run::{CreateRunArgs, Run, RunStatus};
use crate::task::{Task, TaskGraph, TaskStatus};
use crate::worker::Preamble;
use anyhow::Result;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tracing::{info, warn};
use uuid::Uuid;

pub struct Coordinator {
    run: Run,
    graph: TaskGraph,
    dispatches: HashMap<Uuid, WorkerDispatch>,
    max_concurrent: usize,
    event_tx: mpsc::UnboundedSender<CoordinatorEvent>,
}

#[derive(Debug, Clone)]
pub enum CoordinatorEvent {
    TaskDispatched { task_id: Uuid, dispatch_id: Uuid },
    TaskCompleted { task_id: Uuid, result: String },
    TaskFailed { task_id: Uuid, error: String },
    WorkerHeartbeat { dispatch_id: Uuid },
    WorkerDone { dispatch_id: Uuid, result: String },
    Converged { state: ConvergenceState },
}

impl Coordinator {
    pub fn new(args: CreateRunArgs) -> (Self, mpsc::UnboundedReceiver<CoordinatorEvent>) {
        let run_id = Uuid::new_v4();
        let max_concurrent = args.max_concurrent();
        let tasks: Vec<Task> = args
            .task_specs
            .iter()
            .enumerate()
            .map(|(i, spec)| Task {
                id: Uuid::new_v4(),
                run_id,
                spec: spec.clone(),
                status: TaskStatus::Ready,
                deps: Vec::new(),
                parent_id: None,
                result: None,
                error: None,
            })
            .collect();

        let run = Run {
            id: run_id,
            objective: args.objective,
            status: RunStatus::Running,
            max_concurrent,
            created_at: chrono::Utc::now(),
            completed_at: None,
        };

        let (event_tx, event_rx) = mpsc::unbounded_channel();

        let coordinator = Coordinator {
            run,
            graph: TaskGraph { run_id, tasks },
            dispatches: HashMap::new(),
            max_concurrent,
            event_tx,
        };

        (coordinator, event_rx)
    }

    pub fn tick(&mut self) -> Result<()> {
        self.dispatch_ready_tasks()?;
        self.check_convergence();
        Ok(())
    }

    fn dispatch_ready_tasks(&mut self) -> Result<()> {
        let ready_ids: Vec<Uuid> = self
            .graph
            .tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Ready)
            .map(|t| t.id)
            .collect();
        let active_count = self
            .dispatches
            .values()
            .filter(|d| d.is_alive())
            .count();
        let available_slots = self.max_concurrent.saturating_sub(active_count);

        for task_id in ready_ids.into_iter().take(available_slots) {
            let dispatch_id = Uuid::new_v4();
            let dispatch = WorkerDispatch {
                dispatch_id,
                task_id,
                worker_id: format!("worker-{}", dispatch_id),
                worktree_path: None,
                state: DispatchState::Starting,
                created_at: chrono::Utc::now(),
                last_heartbeat: Some(chrono::Utc::now()),
            };

            info!(
                task_id = %task_id,
                dispatch_id = %dispatch_id,
                "Dispatching task"
            );

            self.dispatches.insert(dispatch_id, dispatch);
            if let Some(task) = self.graph.tasks.iter_mut().find(|t| t.id == task_id) {
                task.status = TaskStatus::Dispatched;
            }

            let _ = self.event_tx.send(CoordinatorEvent::TaskDispatched {
                task_id,
                dispatch_id,
            });
        }

        Ok(())
    }

    fn check_convergence(&mut self) {
        let convergence = DagConvergence::evaluate(&self.graph.tasks);
        if matches!(
            convergence.state,
            ConvergenceState::AllDone | ConvergenceState::Empty
        ) {
            self.run.status = RunStatus::Converged;
            self.run.completed_at = Some(chrono::Utc::now());
            let _ = self
                .event_tx
                .send(CoordinatorEvent::Converged {
                    state: convergence.state,
                });
        } else if matches!(convergence.state, ConvergenceState::Stuck) {
            warn!("Orchestration is stuck — blocked tasks with no active workers");
        }
    }

    pub fn handle_worker_done(&mut self, dispatch_id: Uuid, result: String) -> Result<()> {
        if let Some(dispatch) = self.dispatches.get_mut(&dispatch_id) {
            dispatch.state = DispatchState::Succeeded;
            let task_id = dispatch.task_id;

            if let Some(task) = self.graph.tasks.iter_mut().find(|t| t.id == task_id) {
                task.status = TaskStatus::Completed;
                task.result = Some(result.clone());
            }

            let _ = self.event_tx.send(CoordinatorEvent::TaskCompleted {
                task_id,
                result,
            });
        }
        Ok(())
    }

    pub fn handle_worker_failed(&mut self, dispatch_id: Uuid, error: String) -> Result<()> {
        if let Some(dispatch) = self.dispatches.get_mut(&dispatch_id) {
            dispatch.state = DispatchState::Failed;
            let task_id = dispatch.task_id;

            if let Some(task) = self.graph.tasks.iter_mut().find(|t| t.id == task_id) {
                task.status = TaskStatus::Failed;
                task.error = Some(error.clone());
            }

            let _ = self.event_tx.send(CoordinatorEvent::TaskFailed {
                task_id,
                error,
            });
        }
        Ok(())
    }

    pub fn build_preamble(&self, task: &Task, dispatch: &WorkerDispatch) -> Preamble {
        Preamble {
            task_id: task.id.to_string(),
            dispatch_id: dispatch.dispatch_id.to_string(),
            coordinator_handle: self.run.id.to_string(),
            worker_handle: dispatch.worker_id.clone(),
            objective: task.spec.clone(),
            cli_examples: vec![
                "trex-cli orchestration worker-done <dispatch_id> <result>".to_string(),
                "trex-cli orchestration heartbeat <dispatch_id>".to_string(),
                "trex-cli orchestration ask <dispatch_id> <question>".to_string(),
            ],
            base_drift_info: None,
            max_depth: 3,
        }
    }

    pub fn run(&self) -> &Run {
        &self.run
    }

    pub fn graph(&self) -> &TaskGraph {
        &self.graph
    }
}
