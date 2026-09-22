use trex_orchestration::{ConvergenceState, CreateRunArgs, Coordinator, CoordinatorEvent, DagConvergence, RunStatus, Task, TaskStatus};

fn spec_run(specs: &[&str], max_concurrent: Option<usize>) -> (Coordinator, tokio::sync::mpsc::UnboundedReceiver<CoordinatorEvent>) {
    Coordinator::new(CreateRunArgs {
        objective: "ship the feature".to_string(),
        max_concurrent,
        task_specs: specs.iter().map(|s| s.to_string()).collect(),
    })
}

#[test]
fn running_tasks_become_dispatched() {
    let (mut c, mut rx) = spec_run(&["a", "b", "c"], Some(2));
    assert_eq!(c.run().status, RunStatus::Running);
    assert_eq!(c.graph().tasks.len(), 3);
    assert!(c.graph().ready_tasks().len() == 3);

    c.tick().unwrap();
    assert_eq!(
        c.graph().tasks.iter().filter(|t| t.status == TaskStatus::Dispatched).count(),
        2
    );

    let mut dispatched = 0;
    while let Ok(event) = rx.try_recv() {
        if matches!(event, CoordinatorEvent::TaskDispatched { .. }) {
            dispatched += 1;
        }
    }
    assert_eq!(dispatched, 2);
}

#[test]
fn completing_workers_converges_the_run() {
    let (mut c, mut rx) = spec_run(&["a", "b"], None);
    assert_eq!(c.run().max_concurrent, 4);

    c.tick().unwrap();
    let dispatch_ids: Vec<uuid::Uuid> = std::iter::from_fn(|| rx.try_recv().ok())
        .filter_map(|e| match e {
            CoordinatorEvent::TaskDispatched { dispatch_id, .. } => Some(dispatch_id),
            _ => None,
        })
        .collect();
    assert_eq!(dispatch_ids.len(), 2);

    for id in dispatch_ids {
        c.handle_worker_done(id, "done".to_string()).unwrap();
    }
    assert_eq!(c.graph().tasks.iter().filter(|t| t.status == TaskStatus::Completed).count(), 2);

    c.tick().unwrap();
    assert_eq!(c.run().status, RunStatus::Converged);
    assert!(c.run().completed_at.is_some());
    assert!(c.graph().is_converged());
    assert!(!c.graph().has_active());

    let converged = std::iter::from_fn(|| rx.try_recv().ok())
        .filter_map(|e| match e {
            CoordinatorEvent::Converged { state } => Some(state),
            _ => None,
        })
        .next();
    assert_eq!(converged, Some(ConvergenceState::AllDone));
}

#[test]
fn failed_worker_marks_the_task_failed_and_still_converges() {
    let (mut c, mut rx) = spec_run(&["a"], Some(1));
    c.tick().unwrap();

    let dispatch_id = std::iter::from_fn(|| rx.try_recv().ok())
        .find_map(|e| match e {
            CoordinatorEvent::TaskDispatched { dispatch_id, .. } => Some(dispatch_id),
            _ => None,
        })
        .unwrap();

    c.handle_worker_failed(dispatch_id, "timeout".to_string()).unwrap();
    assert_eq!(c.graph().tasks[0].status, TaskStatus::Failed);
    assert_eq!(c.graph().tasks[0].error.as_deref(), Some("timeout"));

    c.tick().unwrap();
    assert_eq!(c.run().status, RunStatus::Converged);
}

#[test]
fn empty_run_converges_immediately() {
    let (mut c, mut rx) = spec_run(&[], Some(1));
    c.tick().unwrap();
    assert_eq!(c.run().status, RunStatus::Converged);

    let converged = std::iter::from_fn(|| rx.try_recv().ok())
        .filter_map(|e| match e {
            CoordinatorEvent::Converged { state } => Some(state),
            _ => None,
        })
        .next();
    assert_eq!(converged, Some(ConvergenceState::Empty));
}

#[test]
fn worker_results_land_on_the_task() {
    let (mut c, mut rx) = spec_run(&["build"], Some(1));
    c.tick().unwrap();
    let dispatch_id = std::iter::from_fn(|| rx.try_recv().ok())
        .find_map(|e| match e {
            CoordinatorEvent::TaskDispatched { dispatch_id, .. } => Some(dispatch_id),
            _ => None,
        })
        .unwrap();

    c.handle_worker_done(dispatch_id, "ok".to_string()).unwrap();
    assert_eq!(c.graph().tasks[0].result.as_deref(), Some("ok"));
    assert_eq!(c.graph().tasks[0].run_id, c.run().id);
}

#[test]
fn preamble_wires_task_and_dispatch_ids() {
    let (mut c, _rx) = spec_run(&["build"], Some(1));
    c.tick().unwrap();
    let task = &c.graph().tasks[0];
    let dispatch_id = uuid::Uuid::new_v4();
    let dispatch = trex_orchestration::WorkerDispatch {
        dispatch_id,
        task_id: task.id,
        worker_id: "worker-x".to_string(),
        worktree_path: None,
        state: trex_orchestration::DispatchState::Starting,
        created_at: chrono::Utc::now(),
        last_heartbeat: Some(chrono::Utc::now()),
    };
    let preamble = c.build_preamble(task, &dispatch);
    assert_eq!(preamble.task_id, task.id.to_string());
    assert_eq!(preamble.dispatch_id, dispatch_id.to_string());
    assert_eq!(preamble.worker_handle, "worker-x");
    assert_eq!(preamble.coordinator_handle, c.run().id.to_string());
}

#[test]
fn dag_convergence_classifies_mixed_graphs() {
    fn task(status: TaskStatus) -> Task {
        Task {
            id: uuid::Uuid::new_v4(),
            run_id: uuid::Uuid::new_v4(),
            spec: String::new(),
            status,
            deps: Vec::new(),
            parent_id: None,
            result: None,
            error: None,
        }
    }

    let all_done = DagConvergence::evaluate(&[task(TaskStatus::Completed), task(TaskStatus::Failed)]);
    assert_eq!(all_done.state, ConvergenceState::AllDone);
    assert_eq!(all_done.completed, 1);
    assert_eq!(all_done.failed, 1);

    let stuck = DagConvergence::evaluate(&[task(TaskStatus::Blocked), task(TaskStatus::Completed)]);
    assert_eq!(stuck.state, ConvergenceState::Stuck);

    let active = DagConvergence::evaluate(&[task(TaskStatus::Dispatched), task(TaskStatus::Ready)]);
    assert_eq!(active.state, ConvergenceState::Active);

    let empty = DagConvergence::evaluate(&[]);
    assert_eq!(empty.state, ConvergenceState::Empty);
}