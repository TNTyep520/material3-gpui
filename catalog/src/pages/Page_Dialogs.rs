// Copyright (c) 2026 TNTyep520
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use gpui::{App, Entity, IntoElement, Render, Styled, Window, div, prelude::*, px};
use material3_gpui::prelude::*;

use super::{gallery, showcase_group};

pub struct DialogsPage {
    pub b_dialog: Entity<ButtonState>,
    pub b_help: Entity<ButtonState>,
    pub dlg_cancel: Entity<ButtonState>,
    pub dlg_ok: Entity<ButtonState>,

    pub(crate) on_open_dialog: Option<super::PageCallback<()>>,
}

impl DialogsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let b_dialog = Button::new("b-dialog", "Delete recordings")
            .filled()
            .build(cx);
        let b_help = Button::new("b-dialog-help", "How dialogs work")
            .tonal()
            .on_click(|_, window, cx| {
                show_snackbar(
                    window,
                    cx,
                    Snackbar::new("Dialogs mount at the window root"),
                    None,
                );
            })
            .build(cx);
        let dlg_cancel = Button::new("dlg-cancel", "Cancel").text().build(cx);
        let dlg_ok = Button::new("dlg-ok", "Delete").text().build(cx);

        cx.new(|_| Self {
            b_dialog,
            b_help,
            dlg_cancel,
            dlg_ok,
            on_open_dialog: None,
        })
    }

    pub fn set_on_open_dialog(&mut self, handler: super::PageCallback<()>) {
        self.on_open_dialog = Some(handler);
    }
}

impl Render for DialogsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = *theme.colors();
        let typography = *theme.typography();

        gallery([showcase_group(
            cx,
            "Dialog launchers",
            [div()
                .w_full()
                .min_w_0()
                .flex()
                .flex_col()
                .gap(px(12.))
                .child(
                    typography
                        .body_medium
                        .apply(div())
                        .text_color(colors.on_surface_variant)
                        .child(
                            "Destructive actions pause on a modal dialog first. Tap \
                             scrim or Cancel to dismiss, Delete to confirm.",
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .items_center()
                        .gap(px(8.))
                        .child(self.b_dialog.clone())
                        .child(self.b_help.clone()),
                )
                .into_any_element()],
        )])
    }
}
