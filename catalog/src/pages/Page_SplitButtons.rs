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

use gpui::{AnyElement, App, AppContext as _, Entity, IntoElement};

use material3_gpui::prelude::*;

use super::{ComponentPage, ExampleMeta, example_content, example_row};

pub struct SplitButtonsPage {
    menu: Entity<MenuState>,
}

impl SplitButtonsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let menu = MenuState::new()
            .item(MenuItem::new("Save as copy"))
            .item(MenuItem::new("Duplicate row"))
            .item(MenuItem::new("Discard"))
            .build(cx);
        cx.new(|_| Self { menu })
    }
}

impl ComponentPage for SplitButtonsPage {
    fn description(&self) -> &'static str {
        "Split buttons let user perform additional actions besides the main action"
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "FilledSplitButtonSample",
                description: "Split Button examples",
            },
            ExampleMeta {
                name: "OutlinedSplitButtonSample",
                description: "Split Button examples",
            },
            ExampleMeta {
                name: "TonalSplitButtonSample",
                description: "Split Button examples",
            },
            ExampleMeta {
                name: "ElevatedSplitButtonSample",
                description: "Split Button examples",
            },
            ExampleMeta {
                name: "SplitButtonWithIconSample",
                description: "Split Button examples",
            },
            ExampleMeta {
                name: "SplitButtonWithDropdownMenuSample",
                description: "Split Button examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => example_content([example_row(
                cx,
                "Filled",
                [SplitButton::new("split-filled", "Filled")
                    .filled()
                    .on_click(|_, window, cx| {
                        show_snackbar(window, cx, Snackbar::new("Split button pressed"), None);
                    })
                    .into_any_element()],
            )]),
            1 => example_content([example_row(
                cx,
                "Outlined",
                [SplitButton::new("split-outlined", "Outlined")
                    .outlined()
                    .on_click(|_, window, cx| {
                        show_snackbar(window, cx, Snackbar::new("Split button pressed"), None);
                    })
                    .into_any_element()],
            )]),
            2 => example_content([example_row(
                cx,
                "Tonal",
                [SplitButton::new("split-tonal", "Tonal")
                    .tonal()
                    .on_click(|_, window, cx| {
                        show_snackbar(window, cx, Snackbar::new("Split button pressed"), None);
                    })
                    .into_any_element()],
            )]),
            3 => example_content([example_row(
                cx,
                "Elevated",
                [SplitButton::new("split-elevated", "Elevated")
                    .variant(ButtonVariant::Elevated)
                    .on_click(|_, window, cx| {
                        show_snackbar(window, cx, Snackbar::new("Split button pressed"), None);
                    })
                    .into_any_element()],
            )]),
            4 => example_content([example_row(
                cx,
                "Trailing icon",
                [SplitButton::new("split-icon", "Icon")
                    .filled()
                    .trailing_icon(IconName::new("add"))
                    .on_click(|_, window, cx| {
                        show_snackbar(window, cx, Snackbar::new("Split button pressed"), None);
                    })
                    .into_any_element()],
            )]),
            5 => example_content([example_row(
                cx,
                "Dropdown menu",
                [SplitButton::new("split-menu", "Menu")
                    .filled()
                    .menu(self.menu.clone())
                    .on_click(|_, window, cx| {
                        show_snackbar(window, cx, Snackbar::new("Split button pressed"), None);
                    })
                    .into_any_element()],
            )]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
