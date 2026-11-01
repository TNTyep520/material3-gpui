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

use gpui::{AnyElement, App, AppContext as _, Entity, IntoElement, ParentElement, Styled, div, px};

use material3_gpui::prelude::*;

use super::{ComponentPage, ExampleMeta, example_content};

pub struct DialogsPage {
    pub b_dialog: Entity<ButtonState>,
    pub b_alert: Entity<ButtonState>,
    pub b_basic: Entity<ButtonState>,
    pub b_icon: Entity<ButtonState>,
    pub dlg_cancel: Entity<ButtonState>,
    pub dlg_ok: Entity<ButtonState>,

    pub(crate) on_open_dialog: Option<super::PageCallback<()>>,
}

impl DialogsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let b_dialog = Button::new("b-dialog", "Show dialog").filled().build(cx);
        let b_alert = Button::new("b-dialog-alert", "Alert dialog")
            .tonal()
            .build(cx);
        let b_basic = Button::new("b-dialog-basic", "Basic dialog")
            .outlined()
            .build(cx);
        let b_icon = Button::new("b-dialog-icon", "With icon")
            .outlined()
            .build(cx);
        let dlg_cancel = Button::new("dlg-cancel", "Cancel").text().build(cx);
        let dlg_ok = Button::new("dlg-ok", "Delete").text().build(cx);

        cx.new(|_| Self {
            b_dialog,
            b_alert,
            b_basic,
            b_icon,
            dlg_cancel,
            dlg_ok,
            on_open_dialog: None,
        })
    }

    /// 打开窗口根对话框的三个启动按钮(AlertDialog / WithIcon / Basic 共用同一 Dialog)。
    pub fn launchers(&self) -> Vec<Entity<ButtonState>> {
        vec![
            self.b_dialog.clone(),
            self.b_alert.clone(),
            self.b_basic.clone(),
        ]
    }

    pub fn set_on_open_dialog(&mut self, handler: super::PageCallback<()>) {
        self.on_open_dialog = Some(handler);
    }
}

impl ComponentPage for DialogsPage {
    fn description(&self) -> &'static str {
        "Dialogs provide important prompts in a user flow. They can require an action, \
         communicate information, or help users accomplish a task."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "AlertDialogSample",
                description: "Dialog examples",
            },
            ExampleMeta {
                name: "AlertDialogWithIconSample",
                description: "Dialog examples",
            },
            ExampleMeta {
                name: "BasicAlertDialogSample",
                description: "Dialog examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => {
                let theme = cx.theme();
                example_content([
                    theme
                        .typography()
                        .body_medium
                        .apply(div())
                        .text_color(theme.colors().on_surface_variant)
                        .child(
                            "Destructive actions pause on a modal dialog first. Tap \
                             scrim or Cancel to dismiss, Delete to confirm.",
                        )
                        .into_any_element(),
                    self.b_dialog.clone().into_any_element(),
                ])
            }
            1 => example_content([self.b_icon.clone().into_any_element()]),
            2 => example_content([div()
                .w_full()
                .min_w_0()
                .flex()
                .flex_wrap()
                .gap(px(8.))
                .child(self.b_alert.clone())
                .child(self.b_basic.clone())
                .into_any_element()]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
