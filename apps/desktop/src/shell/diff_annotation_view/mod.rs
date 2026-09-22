//! Diff Annotation panel — rendered when the Diff Comments pane tab is active.
//!
//! Manages per-line comments on diffs for agent consumption.
//! Stub implementation — full UI to be built out.

use gpui::{
    App, Context, FocusHandle, Focusable, IntoElement, ParentElement, Render, Styled,
    Window, div, px,
};
use trex_diff_annotate::DiffAnnotator;
use trex_settings::{Density, Theme, Typography};

pub struct DiffAnnotationView {
    focus_handle: FocusHandle,
    annotator: DiffAnnotator,
    theme: Theme,
    density: Density,
    typography: Typography,
}

impl DiffAnnotationView {
    pub fn new(
        theme: Theme,
        density: Density,
        typography: Typography,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            annotator: DiffAnnotator::new(),
            theme,
            density,
            typography,
        }
    }
}

impl Focusable for DiffAnnotationView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DiffAnnotationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        trex_settings::appearance::sync(&mut self.theme, &mut self.density, &mut self.typography, cx);
        let theme = self.theme;
        let density = self.density;
        let typography = self.typography.clone();

        let comments = self.annotator.all_comments();

        let mut body = div().flex().flex_col().w_full().flex_1().mt_2().gap(px(density.gap_inline));
        if comments.is_empty() {
            body = body.child(
                div()
                    .text_size(px(typography.t_sub_label))
                    .text_color(theme.fg_muted)
                    .child("No diff comments. Comment on a hunk in the diff view to see it here."),
            );
        } else {
            for comment in comments {
                body = body.child(
                    div()
                        .flex()
                        .flex_col()
                        .text_size(px(typography.t_body_sm))
                        .child(
                            div()
                                .text_color(theme.fg_muted)
                                .child(format!("{}:{}", comment.file_path, comment.line_number)),
                        )
                        .child(div().text_color(theme.fg_base).child(comment.content.clone())),
                );
            }
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
                            .child("Diff Annotations"),
                    )
                    .child(
                        div()
                            .text_size(px(typography.t_sub_label))
                            .text_color(theme.fg_muted)
                            .child(format!("{} comment(s)", comments.len())),
                    ),
            )
            .child(body)
    }
}