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

pub struct ChipsPage {
    chip_assist: Entity<ChipState>,
    chip_assist_elevated: Entity<ChipState>,
    chip_suggestion: Entity<ChipState>,
    chip_suggestion_elevated: Entity<ChipState>,
    chip_filters: Vec<Entity<ChipState>>,
    chip_filter_elevated: Vec<Entity<ChipState>>,
    chip_filter_leading: Entity<ChipState>,
    chip_filter_trailing: Entity<ChipState>,

    chip_input: Option<Entity<ChipState>>,
    chip_input_elevated: Option<Entity<ChipState>>,
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
        let chip_assist_elevated = Chip::new("chip-assist-elevated", "Assist · elevated")
            .assist()
            .elevated(true)
            .build(cx);
        let chip_suggestion = Chip::new("chip-suggestion", "Not now")
            .suggestion()
            .build(cx);
        let chip_suggestion_elevated = Chip::new("chip-suggestion-elevated", "Enhance")
            .suggestion()
            .elevated(true)
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
        let chip_filter_elevated = ["Bordered", "Frame", "Matte"]
            .into_iter()
            .enumerate()
            .map(|(ix, label)| {
                Chip::new(("chip-filter-elevated", ix), label)
                    .filter()
                    .elevated(true)
                    .build(cx)
            })
            .collect();
        let chip_filter_leading = Chip::new("chip-filter-leading", "Photos")
            .filter()
            .leading_icon(IconName::new("image"))
            .build(cx);
        let chip_filter_trailing = Chip::new("chip-filter-trailing", "Receipts")
            .filter()
            .on_remove(|_, window, cx| {
                show_snackbar(window, cx, Snackbar::new("Filter removed"), None);
            })
            .build(cx);

        let page = cx.new(|_| Self {
            chip_assist,
            chip_assist_elevated,
            chip_suggestion,
            chip_suggestion_elevated,
            chip_filters,
            chip_filter_elevated,
            chip_filter_leading,
            chip_filter_trailing,
            chip_input: None,
            chip_input_elevated: None,
        });

        page.update(cx, |this, cx| {
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
            this.chip_input = Some(chip_input);
            cx.notify();
        });
        page.update(cx, |this, cx| {
            let page_entity = cx.entity();
            let chip_input = Chip::new("chip-input-elevated", "With avatar")
                .input()
                .elevated(true)
                .leading_icon(IconName::new("person"))
                .on_remove(move |_, window, cx| {
                    page_entity.update(cx, |page, cx| {
                        page.chip_input_elevated = None;
                        cx.notify();
                    });
                    show_snackbar(window, cx, Snackbar::new("Chip removed"), None);
                })
                .build(cx);
            this.chip_input_elevated = Some(chip_input);
            cx.notify();
        });
        page
    }
}

impl ComponentPage for ChipsPage {
    fn description(&self) -> &'static str {
        "Chips allow users to enter information, make selections, filter content, or \
         trigger actions."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "AssistChipSample",
                description: "Chips examples",
            },
            ExampleMeta {
                name: "ElevatedAssistChipSample",
                description: "Chips examples",
            },
            ExampleMeta {
                name: "FilterChipSample",
                description: "Chips examples",
            },
            ExampleMeta {
                name: "ElevatedFilterChipSample",
                description: "Chips examples",
            },
            ExampleMeta {
                name: "FilterChipWithLeadingIconSample",
                description: "Chips examples",
            },
            ExampleMeta {
                name: "FilterChipWithTrailingIconSample",
                description: "Chips examples",
            },
            ExampleMeta {
                name: "InputChipSample",
                description: "Chips examples",
            },
            ExampleMeta {
                name: "InputChipWithAvatarSample",
                description: "Chips examples",
            },
            ExampleMeta {
                name: "SuggestionChipSample",
                description: "Chips examples",
            },
            ExampleMeta {
                name: "ElevatedSuggestionChipSample",
                description: "Chips examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => example_content([example_row(
                cx,
                "Assist",
                [self.chip_assist.clone().into_any_element()],
            )]),
            1 => example_content([example_row(
                cx,
                "Elevated",
                [self.chip_assist_elevated.clone().into_any_element()],
            )]),
            2 => example_content(
                self.chip_filters
                    .iter()
                    .map(|chip| example_row(cx, "Selectable", [chip.clone().into_any_element()]))
                    .collect::<Vec<_>>(),
            ),
            3 => example_content(
                self.chip_filter_elevated
                    .iter()
                    .map(|chip| example_row(cx, "Elevated", [chip.clone().into_any_element()]))
                    .collect::<Vec<_>>(),
            ),
            4 => example_content([example_row(
                cx,
                "Leading icon",
                [self.chip_filter_leading.clone().into_any_element()],
            )]),
            5 => example_content([example_row(
                cx,
                "Trailing icon",
                [self.chip_filter_trailing.clone().into_any_element()],
            )]),
            6 => example_content(
                self.chip_input
                    .iter()
                    .map(|chip| example_row(cx, "Removable", [chip.clone().into_any_element()]))
                    .collect::<Vec<_>>(),
            ),
            7 => example_content(
                self.chip_input_elevated
                    .iter()
                    .map(|chip| example_row(cx, "Removable", [chip.clone().into_any_element()]))
                    .collect::<Vec<_>>(),
            ),
            8 => example_content([example_row(
                cx,
                "Suggestion",
                [self.chip_suggestion.clone().into_any_element()],
            )]),
            9 => example_content([example_row(
                cx,
                "Elevated",
                [self.chip_suggestion_elevated.clone().into_any_element()],
            )]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
