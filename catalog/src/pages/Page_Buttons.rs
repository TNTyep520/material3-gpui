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

use gpui::{App, Entity, IntoElement, Render, Window, prelude::*};
use material3_gpui::prelude::*;

use super::{gallery, showcase_group, specimen};

pub struct ButtonsPage {
    b_filled: Entity<ButtonState>,
    b_tonal: Entity<ButtonState>,
    b_elevated: Entity<ButtonState>,
    b_outlined: Entity<ButtonState>,
    b_text: Entity<ButtonState>,
    b_icon: Entity<ButtonState>,
    b_disabled: Entity<ButtonState>,
    b_trailing: Entity<ButtonState>,
}

impl ButtonsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let b_filled = Button::new("b-filled", "Book now")
            .filled()
            .on_click(|_, window, cx| {
                show_snackbar(window, cx, Snackbar::new("Trip booked · gate B7"), None);
            })
            .build(cx);
        let b_tonal = FilledTonalButton::new("b-tonal", "Save trip").build(cx);
        let b_elevated = ElevatedButton::new("b-elevated", "Elevated").build(cx);
        let b_outlined = OutlinedButton::new("b-outlined", "Compare fares").build(cx);
        let b_text = TextButton::new("b-text", "Skip").build(cx);
        let b_icon = Button::new("b-icon", "Add itinerary")
            .leading_icon(IconName::Add)
            .build(cx);
        let b_disabled = FilledTonalButton::new("b-disabled", "Sold out")
            .enabled(false)
            .build(cx);
        let b_trailing = Button::new("b-trailing", "Continue")
            .trailing_icon(IconName::ArrowForward)
            .build(cx);

        cx.new(|_| Self {
            b_filled,
            b_tonal,
            b_elevated,
            b_outlined,
            b_text,
            b_icon,
            b_disabled,
            b_trailing,
        })
    }
}

impl Render for ButtonsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        gallery([
            showcase_group(
                cx,
                "Variants",
                [
                    specimen(cx, "Filled", self.b_filled.clone()),
                    specimen(cx, "Tonal", self.b_tonal.clone()),
                    specimen(cx, "Elevated", self.b_elevated.clone()),
                    specimen(cx, "Outlined", self.b_outlined.clone()),
                    specimen(cx, "Text", self.b_text.clone()),
                ],
            ),
            showcase_group(
                cx,
                "States & content",
                [
                    specimen(cx, "Leading icon", self.b_icon.clone()),
                    specimen(cx, "Trailing icon", self.b_trailing.clone()),
                    specimen(cx, "Disabled", self.b_disabled.clone()),
                ],
            ),
        ])
    }
}
