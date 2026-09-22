//! Orchestration page — rendered when the Orchestration pane tab is active.
//!
//! Shows the active `trex_orchestration` run: objective, status, and the
//! task graph the coordinator is driving. A demo run is seeded in
//! construction until real run-creation UI lands; everything rendered is a
//! live read of the `Coordinator`, so the panel grows honestly.

use gpui::{
    App, Context, FocusHandle, Focusable, IntoElement, ParentElement, Render, Styled,
    Window, div, px,
};
use trex_orchestration::{Coordinator, CreateRunArgs, RunStatus, TaskStatus};
use trex_settings::{Density, Theme, Typography};

pub struct OrchestrationView {
    focus_handle: FocusHandle,
    coordinator: Coordinator,
    theme: Theme,
    density: Density,
    typography: Typography,
}

impl OrchestrationView {
    /// Seed one demo run so the panel has a real graph to show. Task-creation
    /// UI lands in a later phase; this only proves the integration.
    fn seed_coordinator() -> Coordinator {
        let (mut coordinator, _event_rx) = Coordinator::new(CreateRunArgs {
            objective: "Scaffold the next release".into(),
            max_concurrent: Some(2),
            task_specs: vec![
                "Map the task graph".into(),
                "Dispatch two workers".into(),
                "Converge on the result".into(),
            ],
        });
        // One tick marks up to `max_concurrent` tasks Dispatched, so the
        // worker status section has something besides "Ready" to draw.
        let _ = coordinator.tick();
        coordinator
    }

    pub fn new(
        theme: Theme,
        density: Density,
        typography: Typography,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            coordinator: Self::seed_coordinator(),
            theme,
            density,
            typography,
        }
    }
}

impl Focusable for OrchestrationView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl OrchestrationView {
    fn status_label(status: &RunStatus) -> &'static str {
        match status {
            RunStatus::Created => "Created",
            RunStatus::Running => "Running",
            RunStatus::Converged => "Converged",
            RunStatus::Failed => "Failed",
        }
    }

    fn task_status_label(status: &TaskStatus) -> &'static str {
        match status {
            TaskStatus::Pending => "Pending",
            TaskStatus::Ready => "Ready",
            TaskStatus::Dispatched => "Dispatched",
            TaskStatus::Completed => "Completed",
            TaskStatus::Failed => "Failed",
            TaskStatus::Blocked => "Blocked",
        }
    }
}

impl Render for OrchestrationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        trex_settings::appearance::sync(&mut self.theme, &mut self.density, &mut self.typography, cx);
        let theme = self.theme;
        let density = self.density;
        let typography = self.typography.clone();
        let run = self.coordinator.run();
        let tasks = &self.coordinator.graph().tasks;

        let mut task_col = div().flex().flex_col().w_full().mt_2().gap(px(density.gap_inline));
        for task in tasks {
            task_col = task_col.child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .text_size(px(typography.t_body_sm))
                    .child(div().text_color(theme.fg_base).child(task.spec.clone()))
                    .child(
                        div()
                            .text_color(theme.fg_muted)
                            .child(Self::task_status_label(&task.status)),
                    ),
            );
        }

        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(theme.bg_panel)
            .p(px(density.pad_panel))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(typography.t_body_md))
                            .font_weight(typography.w_semibold)
                            .text_color(theme.fg_base)
                            .child("Orchestration"),
                    )
                    .child(
                        div()
                            .text_size(px(typography.t_sub_label))
                            .text_color(theme.fg_muted)
                            .child(Self::status_label(&run.status)),
                    ),
            )
            .child(
                div()
                    .mt_1()
                    .text_size(px(typography.t_sub_label))
                    .text_color(theme.fg_muted)
                    .child(run.objective.clone()),
            )
            .child(task_col)
    }
}