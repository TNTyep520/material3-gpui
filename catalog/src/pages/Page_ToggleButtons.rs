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
use gpui::{AnyElement, App, AppContext as _, Entity, IntoElement, Window};

use material3_gpui::prelude::*;

use super::{ComponentPage, ExampleMeta, example_content, example_row};

/// 切换回调统一展示 Snackbar,文案随勾选状态变化。
fn toggle_snackbar(checked: bool, window: &mut Window, cx: &mut App) {
    let message = if checked { "Toggle on" } else { "Toggle off" };
    show_snackbar(window, cx, Snackbar::new(message), None);
}

pub struct ToggleButtonsPage {
    tb_unchecked: Entity<ToggleButtonState>,
    tb_checked: Entity<ToggleButtonState>,
    tb_icon: Entity<ToggleButtonState>,
    tb_tonal: Entity<ToggleButtonState>,
    tb_outlined: Entity<ToggleButtonState>,
    tb_elevated: Entity<ToggleButtonState>,
}

impl ToggleButtonsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let tb_unchecked = ToggleButton::new("tb-unchecked", "Toggle")
            .on_change(toggle_snackbar)
            .build(cx);
        let tb_checked = ToggleButton::new("tb-checked", "Toggle")
            .checked(true)
            .on_change(toggle_snackbar)
            .build(cx);
        let tb_icon = ToggleButton::new("tb-icon", "Toggle")
            .icon(IconName::new("favorite"))
            .checked(true)
            .on_change(toggle_snackbar)
            .build(cx);
        let tb_tonal = ToggleButton::new("tb-tonal", "Toggle")
            .checked(true)
            .variant(ToggleButtonVariant::FilledTonal)
            .on_change(toggle_snackbar)
            .build(cx);
        let tb_outlined = ToggleButton::new("tb-outlined", "Toggle")
            .variant(ToggleButtonVariant::Outlined)
            .on_change(toggle_snackbar)
            .build(cx);
        let tb_elevated = ToggleButton::new("tb-elevated", "Toggle")
            .variant(ToggleButtonVariant::Elevated)
            .on_change(toggle_snackbar)
            .build(cx);

        cx.new(|_| Self {
            tb_unchecked,
            tb_checked,
            tb_icon,
            tb_tonal,
            tb_outlined,
            tb_elevated,
        })
    }
}

impl ComponentPage for ToggleButtonsPage {
    fn description(&self) -> &'static str {
        "Toggle buttons provide a selectable button that animates on press."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "ToggleButtonSample",
                description: "ToggleButton examples",
            },
            ExampleMeta {
                name: "ToggleButtonWithIconSample",
                description: "ToggleButton examples",
            },
            ExampleMeta {
                name: "FilledTonalToggleButtonSample",
                description: "ToggleButton examples",
            },
            ExampleMeta {
                name: "OutlinedToggleButtonSample",
                description: "ToggleButton examples",
            },
            ExampleMeta {
                name: "ElevatedToggleButtonSample",
                description: "ToggleButton examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => example_content([
                example_row(
                    cx,
                    "Unchecked",
                    [self.tb_unchecked.clone().into_any_element()],
                ),
                example_row(cx, "Checked", [self.tb_checked.clone().into_any_element()]),
            ]),
            1 => example_content([example_row(
                cx,
                "With icon",
                [self.tb_icon.clone().into_any_element()],
            )]),
            2 => example_content([example_row(
                cx,
                "Filled tonal",
                [self.tb_tonal.clone().into_any_element()],
            )]),
            3 => example_content([example_row(
                cx,
                "Outlined",
                [self.tb_outlined.clone().into_any_element()],
            )]),
            4 => example_content([example_row(
                cx,
                "Elevated",
                [self.tb_elevated.clone().into_any_element()],
            )]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
