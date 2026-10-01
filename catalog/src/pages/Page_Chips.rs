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

use gpui::{App, AppContext as _, Entity, IntoElement, Render, Window};
use material3_gpui::prelude::*;

use super::{gallery, showcase_group, specimen};

pub struct ChipsPage {
    chip_assist: Entity<ChipState>,
    chip_filters: Vec<Entity<ChipState>>,

    chip_input: Option<Entity<ChipState>>,
    chip_suggestion: Entity<ChipState>,
}

impl ChipsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let chip_assist = Chip::new("chip-assist", "Draft reply")
            .assist()
            .leading_icon(IconName::new("edit"))
            .on_click(|_, window, cx| {
                show_snackbar(window, cx, Snackbar::new("Reply drafted"), None);
            })
            .build(cx);
        let chip_filters = ["Photos", "Receipts", "Travel"]
            .into_iter()
            .enumerate()
            .map(|(ix, label)| {
                Chip::new(("chip-filter", ix), label)
                    .filter()
                    .on_click(move |_, window, cx| {
                        show_snackbar(
                            window,
                            cx,
                            Snackbar::new(format!("\"{label}\" filter applied")),
                            None,
                        );
                    })
                    .build(cx)
            })
            .collect();
        let chip_suggestion = Chip::new("chip-suggestion", "Not now")
            .suggestion()
            .elevated(true)
            .build(cx);

        let page = cx.new(|_| Self {
            chip_assist,
            chip_filters,
            chip_input: None,
            chip_suggestion,
        });

        page.update(cx, |page, cx| {
            let page_entity = cx.entity();
            let chip_input = Chip::new("chip-input", "On vacation")
                .input()
                .on_remove(move |_, window, cx| {
                    page_entity.update(cx, |page, cx| {
                        page.chip_input = None;
                        cx.notify();
                    });
                    show_snackbar(window, cx, Snackbar::new("Filter removed"), None);
                })
                .build(cx);
            page.chip_input = Some(chip_input);
            cx.notify();
        });
        page
    }
}

impl Render for ChipsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        gallery([
            showcase_group(
                cx,
                "Variants",
                [
                    specimen(cx, "Assist", self.chip_assist.clone()),
                    specimen(cx, "Suggestion · elevated", self.chip_suggestion.clone()),
                ],
            ),
            showcase_group(
                cx,
                "Filter chips",
                self.chip_filters
                    .iter()
                    .map(|chip| specimen(cx, "Selectable", chip.clone())),
            ),
            showcase_group(
                cx,
                "Input chip",
                self.chip_input
                    .iter()
                    .map(|chip| specimen(cx, "Removable", chip.clone())),
            ),
        ])
    }
}
