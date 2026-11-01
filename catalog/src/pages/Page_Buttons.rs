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

pub struct ButtonsPage {
    b_filled: Entity<ButtonState>,
    b_filled_disabled: Entity<ButtonState>,
    b_icon: Entity<ButtonState>,
    b_trailing: Entity<ButtonState>,
    b_elevated: Entity<ButtonState>,
    b_tonal: Entity<ButtonState>,
    b_outlined: Entity<ButtonState>,
    b_text: Entity<ButtonState>,
}

impl ButtonsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let b_filled = Button::new("b-filled", "Filled")
            .filled()
            .on_click(|_, window, cx| {
                show_snackbar(window, cx, Snackbar::new("Filled button pressed"), None);
            })
            .build(cx);
        let b_filled_disabled = Button::new("b-filled-disabled", "Filled")
            .filled()
            .disabled(true)
            .build(cx);
        let b_icon = Button::new("b-icon", "Icon")
            .filled()
            .leading_icon(IconName::new("favorite"))
            .build(cx);
        let b_trailing = Button::new("b-trailing", "Icon")
            .filled()
            .trailing_icon(IconName::new("add"))
            .build(cx);
        let b_elevated = ElevatedButton::new("b-elevated", "Elevated").build(cx);
        let b_tonal = FilledTonalButton::new("b-tonal", "Filled tonal").build(cx);
        let b_outlined = OutlinedButton::new("b-outlined", "Outlined").build(cx);
        let b_text = TextButton::new("b-text", "Text button").build(cx);
        cx.new(|_| Self {
            b_filled,
            b_filled_disabled,
            b_icon,
            b_trailing,
            b_elevated,
            b_tonal,
            b_outlined,
            b_text,
        })
    }
}

impl ComponentPage for ButtonsPage {
    fn description(&self) -> &'static str {
        "Buttons help people initiate actions, from sending an email, to sharing a \
         document, to liking a post."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "ButtonSample",
                description: "Button examples",
            },
            ExampleMeta {
                name: "ButtonWithIconSample",
                description: "Button examples",
            },
            ExampleMeta {
                name: "ElevatedButtonSample",
                description: "Button examples",
            },
            ExampleMeta {
                name: "FilledTonalButtonSample",
                description: "Button examples",
            },
            ExampleMeta {
                name: "OutlinedButtonSample",
                description: "Button examples",
            },
            ExampleMeta {
                name: "TextButtonSample",
                description: "Button examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => example_content([example_row(
                cx,
                "Filled",
                [
                    self.b_filled.clone().into_any_element(),
                    self.b_filled_disabled.clone().into_any_element(),
                ],
            )]),
            1 => example_content([
                example_row(cx, "Leading icon", [self.b_icon.clone().into_any_element()]),
                example_row(
                    cx,
                    "Trailing icon",
                    [self.b_trailing.clone().into_any_element()],
                ),
            ]),
            2 => example_content([example_row(
                cx,
                "Elevated",
                [self.b_elevated.clone().into_any_element()],
            )]),
            3 => example_content([example_row(
                cx,
                "Filled tonal",
                [self.b_tonal.clone().into_any_element()],
            )]),
            4 => example_content([example_row(
                cx,
                "Outlined",
                [self.b_outlined.clone().into_any_element()],
            )]),
            5 => example_content([example_row(
                cx,
                "Text button",
                [self.b_text.clone().into_any_element()],
            )]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
