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

pub struct IconButtonsPage {
    ib_standard: Entity<IconButtonState>,
    ib_filled: Entity<IconButtonState>,
    ib_tonal: Entity<IconButtonState>,
    ib_outlined: Entity<IconButtonState>,
    ib_disabled: Entity<IconButtonState>,
    ib_toggle: Entity<IconButtonState>,
    ib_toggle_selected: Entity<IconButtonState>,
    ib_toggle_filled: Entity<IconButtonState>,
    ib_toggle_outlined: Entity<IconButtonState>,
    ib_toggle_tonal: Entity<IconButtonState>,
    ib_large_square: Entity<IconButtonState>,
    ib_xsmall_square: Entity<IconButtonState>,
}

impl IconButtonsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let ib_standard = IconButton::new("ib-standard", IconName::new("favorite"))
            .on_click(|_, window, cx| {
                show_snackbar(window, cx, Snackbar::new("Favorite set"), None);
            })
            .build(cx);
        let ib_filled = FilledIconButton::new("ib-filled", IconName::new("edit")).build(cx);
        let ib_tonal = FilledTonalIconButton::new("ib-tonal", IconName::new("settings")).build(cx);
        let ib_outlined =
            OutlinedIconButton::new("ib-outlined", IconName::new("more_vert")).build(cx);
        let ib_disabled = OutlinedIconButton::new("ib-disabled", IconName::new("edit"))
            .enabled(false)
            .build(cx);
        let ib_toggle = IconToggleButton::new("ib-toggle", IconName::new("favorite"))
            .on_checked_change(|checked, window, cx| {
                let message = if checked {
                    "Favorite set"
                } else {
                    "Favorite removed"
                };
                show_snackbar(window, cx, Snackbar::new(message), None);
            })
            .build(cx);
        let ib_toggle_selected =
            FilledIconToggleButton::new("ib-toggle-selected", IconName::new("star"))
                .checked(true)
                .build(cx);
        let ib_toggle_filled =
            FilledIconToggleButton::new("ib-toggle-filled", IconName::new("edit")).build(cx);
        let ib_toggle_outlined =
            OutlinedIconToggleButton::new("ib-toggle-outlined", IconName::new("build")).build(cx);
        let ib_toggle_tonal =
            FilledTonalIconToggleButton::new("ib-toggle-tonal", IconName::new("settings"))
                .build(cx);
        let ib_large_square =
            FilledTonalIconButton::new("ib-large-square", IconName::new("settings"))
                .size(IconButtonSize::Large)
                .shape(IconButtonShape::Square)
                .build(cx);
        let ib_xsmall_square = IconButton::new("ib-xsmall-square", IconName::new("edit"))
            .size(IconButtonSize::XSmall)
            .shape(IconButtonShape::Square)
            .build(cx);

        cx.new(|_| Self {
            ib_standard,
            ib_filled,
            ib_tonal,
            ib_outlined,
            ib_disabled,
            ib_toggle,
            ib_toggle_selected,
            ib_toggle_filled,
            ib_toggle_outlined,
            ib_toggle_tonal,
            ib_large_square,
            ib_xsmall_square,
        })
    }
}

impl ComponentPage for IconButtonsPage {
    fn description(&self) -> &'static str {
        "Icon buttons allow users to take actions and make choices with a single tap."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "IconButtonSample",
                description: "Icon button examples",
            },
            ExampleMeta {
                name: "FilledIconButtonSample",
                description: "Icon button examples",
            },
            ExampleMeta {
                name: "FilledTonalIconButtonSample",
                description: "Icon button examples",
            },
            ExampleMeta {
                name: "OutlinedIconButtonSample",
                description: "Icon button examples",
            },
            ExampleMeta {
                name: "IconToggleButtonSample",
                description: "Icon button examples",
            },
            ExampleMeta {
                name: "FilledIconToggleButtonSample",
                description: "Icon button examples",
            },
            ExampleMeta {
                name: "OutlinedIconToggleButtonSample",
                description: "Icon button examples",
            },
            ExampleMeta {
                name: "FilledTonalIconToggleButtonSample",
                description: "Icon button examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => example_content([
                example_row(
                    cx,
                    "Standard",
                    [self.ib_standard.clone().into_any_element()],
                ),
                example_row(
                    cx,
                    "Disabled",
                    [self.ib_disabled.clone().into_any_element()],
                ),
                example_row(
                    cx,
                    "Large square",
                    [self.ib_large_square.clone().into_any_element()],
                ),
                example_row(
                    cx,
                    "XSmall square",
                    [self.ib_xsmall_square.clone().into_any_element()],
                ),
            ]),
            1 => example_content([example_row(
                cx,
                "Filled",
                [self.ib_filled.clone().into_any_element()],
            )]),
            2 => example_content([example_row(
                cx,
                "Tonal",
                [self.ib_tonal.clone().into_any_element()],
            )]),
            3 => example_content([example_row(
                cx,
                "Outlined",
                [self.ib_outlined.clone().into_any_element()],
            )]),
            4 => example_content([example_row(
                cx,
                "Unselected",
                [self.ib_toggle.clone().into_any_element()],
            )]),
            5 => example_content([
                example_row(
                    cx,
                    "Filled",
                    [self.ib_toggle_filled.clone().into_any_element()],
                ),
                example_row(
                    cx,
                    "Selected · filled",
                    [self.ib_toggle_selected.clone().into_any_element()],
                ),
            ]),
            6 => example_content([example_row(
                cx,
                "Outlined",
                [self.ib_toggle_outlined.clone().into_any_element()],
            )]),
            7 => example_content([example_row(
                cx,
                "Tonal",
                [self.ib_toggle_tonal.clone().into_any_element()],
            )]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
