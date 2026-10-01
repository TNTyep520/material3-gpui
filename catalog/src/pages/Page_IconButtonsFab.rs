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

pub struct IconButtonsFabPage {
    ib_standard: Entity<IconButtonState>,
    ib_filled: Entity<IconButtonState>,
    ib_tonal: Entity<IconButtonState>,
    ib_outlined: Entity<IconButtonState>,
    ib_toggle: Entity<IconButtonState>,
    ib_toggle_selected: Entity<IconButtonState>,
    ib_disabled: Entity<IconButtonState>,
    ib_large_square: Entity<IconButtonState>,
    fab_small: Entity<FabState>,
    fab_std: Entity<FabState>,
    fab_ext: Entity<FabState>,
    fab_low: Entity<FabState>,
}

impl IconButtonsFabPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let ib_standard = IconButton::new("ib-standard", IconName::Favorite)
            .on_click(|_, window, cx| {
                show_snackbar(window, cx, Snackbar::new("Favorite set"), None);
            })
            .build(cx);
        let ib_filled = FilledIconButton::new("ib-filled", IconName::Edit).build(cx);
        let ib_tonal = FilledTonalIconButton::new("ib-tonal", IconName::Settings).build(cx);
        let ib_outlined = OutlinedIconButton::new("ib-outlined", IconName::MoreVert).build(cx);
        let ib_toggle = IconToggleButton::new("ib-toggle", IconName::Favorite)
            .on_checked_change(|checked, window, cx| {
                let message = if checked {
                    "Favorite set"
                } else {
                    "Favorite removed"
                };
                show_snackbar(window, cx, Snackbar::new(message), None);
            })
            .build(cx);
        let ib_toggle_selected = FilledIconToggleButton::new("ib-toggle-selected", IconName::Star)
            .checked(true)
            .on_checked_change(|checked, window, cx| {
                let message = if checked { "Starred" } else { "Unstarred" };
                show_snackbar(window, cx, Snackbar::new(message), None);
            })
            .build(cx);
        let ib_disabled = OutlinedIconButton::new("ib-disabled", IconName::Edit)
            .enabled(false)
            .build(cx);
        let ib_large_square = FilledTonalIconButton::new("ib-large-square", IconName::Settings)
            .size(IconButtonSize::Medium)
            .shape(IconButtonShape::Square)
            .build(cx);

        let fab_small = Fab::new("fab-small", IconName::Edit)
            .size(FabSize::Small)
            .on_click(|_, window, cx| {
                show_snackbar(window, cx, Snackbar::new("Small FAB: quick note"), None);
            })
            .build(cx);
        let fab_std = Fab::new("fab-std", IconName::Add)
            .on_click(|_, window, cx| {
                show_snackbar(window, cx, Snackbar::new("Draft started"), None);
            })
            .build(cx);
        let fab_ext = Fab::new("fab-ext", IconName::Add)
            .color(FabColor::Tertiary)
            .label("Compose")
            .on_click(|_, window, cx| {
                show_snackbar(window, cx, Snackbar::new("Compose from anywhere"), None);
            })
            .build(cx);
        let fab_low = Fab::new("fab-low", IconName::Star).lowered(true).build(cx);

        cx.new(|_| Self {
            ib_standard,
            ib_filled,
            ib_tonal,
            ib_outlined,
            ib_toggle,
            ib_toggle_selected,
            ib_disabled,
            ib_large_square,
            fab_small,
            fab_std,
            fab_ext,
            fab_low,
        })
    }
}

impl Render for IconButtonsFabPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        gallery([
            showcase_group(
                cx,
                "Icon button variants",
                [
                    specimen(cx, "Standard", self.ib_standard.clone()),
                    specimen(cx, "Filled", self.ib_filled.clone()),
                    specimen(cx, "Tonal", self.ib_tonal.clone()),
                    specimen(cx, "Outlined", self.ib_outlined.clone()),
                ],
            ),
            showcase_group(
                cx,
                "FAB variants",
                [
                    specimen(cx, "Small", self.fab_small.clone()),
                    specimen(cx, "Standard", self.fab_std.clone()),
                    specimen(cx, "Extended", self.fab_ext.clone()),
                    specimen(cx, "Lowered", self.fab_low.clone()),
                ],
            ),
            showcase_group(
                cx,
                "States & shapes",
                [
                    specimen(cx, "Toggle", self.ib_toggle.clone()),
                    specimen(cx, "Selected", self.ib_toggle_selected.clone()),
                    specimen(cx, "Disabled", self.ib_disabled.clone()),
                    specimen(cx, "Medium square", self.ib_large_square.clone()),
                ],
            ),
        ])
    }
}
