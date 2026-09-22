//! Accounts panel — rendered when the Accounts pane tab is active.
//!
//! Manages API accounts, rate limits, and usage tracking.
//! Stub implementation — full UI to be built out.

use gpui::{
    App, Context, FocusHandle, Focusable, IntoElement, ParentElement, Render, Styled,
    Window, div, px,
};
use trex_accounts::service::AccountService;
use trex_settings::{Density, Theme, Typography};

pub struct AccountsView {
    focus_handle: FocusHandle,
    service: AccountService,
    theme: Theme,
    density: Density,
    typography: Typography,
}

impl AccountsView {
    pub fn new(
        theme: Theme,
        density: Density,
        typography: Typography,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            service: AccountService::new(),
            theme,
            density,
            typography,
        }
    }
}

impl Focusable for AccountsView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for AccountsView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        trex_settings::appearance::sync(&mut self.theme, &mut self.density, &mut self.typography, cx);
        let theme = self.theme;
        let density = self.density;
        let typography = self.typography.clone();

        let census = self.service.list().len();
        let active = self.service.active();

        let mut body = div().flex().flex_col().w_full().flex_1().mt_2().gap(px(density.gap_inline));
        if census == 0 {
            body = body.child(
                div()
                    .text_size(px(typography.t_sub_label))
                    .text_color(theme.fg_muted)
                    .child("No accounts configured. Add one in Settings."),
            );
        } else {
            for account in self.service.list().iter() {
                let is_active = active.is_some_and(|a| a.id == account.id);
                body = body.child(
                    div()
                        .flex()
                        .flex_row()
                        .justify_between()
                        .text_size(px(typography.t_body_sm))
                        .child(div().text_color(theme.fg_base).child(format!("{:?}", account.provider)))
                        .child(
                            div()
                                .text_color(if is_active { theme.fg_base } else { theme.fg_muted })
                                .child(if is_active { "active" } else { "inactive" }),
                        ),
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
                            .child("Accounts"),
                    )
                    .child(
                        div()
                            .text_size(px(typography.t_sub_label))
                            .text_color(theme.fg_muted)
                            .child(format!("{census} configured")),
                    ),
            )
            .child(body)
    }
}