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

pub struct SegmentedButtonsPage {
    single: Entity<SegmentedButtonRowState>,
    multiple: Entity<SegmentedButtonRowState>,
    with_icons: Entity<SegmentedButtonRowState>,
    disabled: Entity<SegmentedButtonRowState>,
}

impl SegmentedButtonsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let single = SegmentedButtonRow::new("seg-single")
            .buttons([
                SegmentedButton::new("Day").selected(true),
                SegmentedButton::new("Week"),
                SegmentedButton::new("Month"),
            ])
            .build(cx);
        let multiple = SegmentedButtonRow::new("seg-multiple")
            .selection_mode(SegmentedButtonSelectionMode::Multiple)
            .buttons([
                SegmentedButton::new("Walk")
                    .icon(IconName::new("directions_walk"))
                    .selected(true),
                SegmentedButton::new("Bike").icon(IconName::new("directions_bike")),
                SegmentedButton::new("Drive")
                    .icon(IconName::new("directions_car"))
                    .selected(true),
            ])
            .build(cx);
        let with_icons = SegmentedButtonRow::new("seg-icons")
            .buttons([
                SegmentedButton::new("Day").icon(IconName::new("calendar_month")),
                SegmentedButton::new("Week").icon(IconName::new("calendar_month")),
                SegmentedButton::new("Month").icon(IconName::new("calendar_month")),
            ])
            .build(cx);
        let disabled = SegmentedButtonRow::new("seg-disabled")
            .disabled(true)
            .buttons([
                SegmentedButton::new("Day").selected(true),
                SegmentedButton::new("Week"),
                SegmentedButton::new("Month"),
            ])
            .build(cx);

        cx.new(|_| Self {
            single,
            multiple,
            with_icons,
            disabled,
        })
    }
}

impl ComponentPage for SegmentedButtonsPage {
    fn description(&self) -> &'static str {
        "Segmented buttons help people select options, switch views, or sort elements."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "SegmentedButtonSingleSelectSample",
                description: "Segmented Button examples",
            },
            ExampleMeta {
                name: "SegmentedButtonMultiSelectSample",
                description: "Segmented Button examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => example_content([example_row(
                cx,
                "Single select",
                [self.single.clone().into_any_element()],
            )]),
            1 => example_content([
                example_row(
                    cx,
                    "Multi select",
                    [self.multiple.clone().into_any_element()],
                ),
                example_row(
                    cx,
                    "With icons",
                    [self.with_icons.clone().into_any_element()],
                ),
                example_row(cx, "Disabled", [self.disabled.clone().into_any_element()]),
            ]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
