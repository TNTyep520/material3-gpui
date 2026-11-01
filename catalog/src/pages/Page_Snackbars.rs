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

use std::time::Duration;

use gpui::{AnyElement, App, AppContext as _, Entity, IntoElement};

use material3_gpui::prelude::*;

use super::{ComponentPage, ExampleMeta, example_content, example_row};

pub struct SnackbarsPage {
    simple: Entity<ButtonState>,
    with_action: Entity<ButtonState>,
    custom: Entity<ButtonState>,
    multiline: Entity<ButtonState>,
    coroutines: Entity<ButtonState>,
}

impl SnackbarsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let simple = Button::new("snack-simple-btn", "Show snackbar")
            .filled()
            .on_click(|_, window, cx| {
                show_snackbar(window, cx, Snackbar::new("Show snackbar"), None);
            })
            .build(cx);
        let with_action = Button::new("snack-action-btn", "Show snackbar")
            .filled()
            .on_click(|_, window, cx| {
                show_snackbar(
                    window,
                    cx,
                    Snackbar::new(" Snackbar # ")
                        .action("UNDO")
                        .on_action(|window, cx| {
                            show_snackbar(window, cx, Snackbar::new("Restored to drafts"), None);
                        }),
                    None,
                );
            })
            .build(cx);
        let custom = Button::new("snack-custom-btn", "Show snackbar")
            .filled()
            .on_click(|_, window, cx| {
                show_snackbar(
                    window,
                    cx,
                    Snackbar::new("Custom Snackbar Demo"),
                    Some(Duration::from_secs(8)),
                );
            })
            .build(cx);
        let multiline = Button::new("snack-multiline-btn", "Show snackbar")
            .filled()
            .on_click(|_, window, cx| {
                show_snackbar(
                    window,
                    cx,
                    Snackbar::new(
                        "Multiline Snackbar Demo: this message is long enough to wrap \
                         onto a second line.",
                    ),
                    None,
                );
            })
            .build(cx);
        let coroutines = Button::new("snack-coroutines-btn", "Show snackbar")
            .filled()
            .on_click(|_, window, cx| {
                show_snackbar(window, cx, Snackbar::new("Show snackbar"), None);
            })
            .build(cx);
        cx.new(|_| Self {
            simple,
            with_action,
            custom,
            multiline,
            coroutines,
        })
    }
}

impl ComponentPage for SnackbarsPage {
    fn description(&self) -> &'static str {
        "Snackbars provide brief messages about app processes at the bottom of the screen."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "ScaffoldWithSimpleSnackbar",
                description: "Snackbars examples",
            },
            ExampleMeta {
                name: "ScaffoldWithIndefiniteSnackbar",
                description: "Snackbars examples",
            },
            ExampleMeta {
                name: "ScaffoldWithCustomSnackbar",
                description: "Snackbars examples",
            },
            ExampleMeta {
                name: "ScaffoldWithMultilineSnackbar",
                description: "Snackbars examples",
            },
            ExampleMeta {
                name: "ScaffoldWithCoroutinesSnackbar",
                description: "Snackbars examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => example_content([example_row(
                cx,
                "Simple snackbar",
                [self.simple.clone().into_any_element()],
            )]),
            1 => example_content([example_row(
                cx,
                "With action",
                [self.with_action.clone().into_any_element()],
            )]),
            2 => example_content([example_row(
                cx,
                "Custom message",
                [self.custom.clone().into_any_element()],
            )]),
            3 => example_content([example_row(
                cx,
                "Multiline",
                [self.multiline.clone().into_any_element()],
            )]),
            4 => example_content([example_row(
                cx,
                "Coroutines snackbar",
                [self.coroutines.clone().into_any_element()],
            )]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
