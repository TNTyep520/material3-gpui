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

pub struct ExtendedFabPage {
    fab_std: Entity<FabState>,
    fab_small: Entity<FabState>,
    fab_medium: Entity<FabState>,
    fab_large: Entity<FabState>,
    fab_icon: Entity<FabState>,
    fab_text_only: Entity<FabState>,
}

impl ExtendedFabPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let fab_std =
            ExtendedFloatingActionButton::new("ext-fab-std", IconName::new("add"), "Compose")
                .on_click(|_, window, cx| {
                    show_snackbar(window, cx, Snackbar::new("Compose from anywhere"), None);
                })
                .build(cx);
        let fab_small =
            SmallExtendedFloatingActionButton::new("ext-fab-small", IconName::new("edit"), "Edit")
                .build(cx);
        let fab_medium = MediumExtendedFloatingActionButton::new(
            "ext-fab-medium",
            IconName::new("image"),
            "Attach",
        )
        .build(cx);
        let fab_large = LargeExtendedFloatingActionButton::new(
            "ext-fab-large",
            IconName::new("favorite"),
            "Like it",
        )
        .build(cx);
        let fab_icon =
            ExtendedFloatingActionButton::new("ext-fab-icon", IconName::new("star"), "Starred")
                .color(FabColor::Tertiary)
                .build(cx);
        let fab_text_only = Fab::new("ext-fab-text", IconName::new("add"))
            .text("New message")
            .build(cx);

        cx.new(|_| Self {
            fab_std,
            fab_small,
            fab_medium,
            fab_large,
            fab_icon,
            fab_text_only,
        })
    }
}

impl ComponentPage for ExtendedFabPage {
    fn description(&self) -> &'static str {
        "Extended FABs help people take primary actions. They're wider than FABs to \
         accommodate a text label and larger target area."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "ExtendedFloatingActionButtonSample",
                description: "Extended FAB examples",
            },
            ExampleMeta {
                name: "SmallExtendedFloatingActionButtonSample",
                description: "Extended FAB examples",
            },
            ExampleMeta {
                name: "MediumExtendedFloatingActionButtonSample",
                description: "Extended FAB examples",
            },
            ExampleMeta {
                name: "LargeExtendedFloatingActionButtonSample",
                description: "Extended FAB examples",
            },
            ExampleMeta {
                name: "ExtendedFloatingActionButtonTextSample",
                description: "Extended FAB examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => example_content([
                example_row(cx, "Standard", [self.fab_std.clone().into_any_element()]),
                example_row(
                    cx,
                    "Tertiary color",
                    [self.fab_icon.clone().into_any_element()],
                ),
            ]),
            1 => example_content([example_row(
                cx,
                "Small",
                [self.fab_small.clone().into_any_element()],
            )]),
            2 => example_content([example_row(
                cx,
                "Medium",
                [self.fab_medium.clone().into_any_element()],
            )]),
            3 => example_content([example_row(
                cx,
                "Large",
                [self.fab_large.clone().into_any_element()],
            )]),
            4 => example_content([example_row(
                cx,
                "Text only",
                [self.fab_text_only.clone().into_any_element()],
            )]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
