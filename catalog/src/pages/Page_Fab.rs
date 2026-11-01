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

pub struct FabPage {
    fab_small: Entity<FabState>,
    fab_std: Entity<FabState>,
    fab_medium: Entity<FabState>,
    fab_large: Entity<FabState>,
    fab_primary: Entity<FabState>,
    fab_secondary: Entity<FabState>,
    fab_tertiary: Entity<FabState>,
    fab_surface: Entity<FabState>,
    fab_lowered: Entity<FabState>,
}

impl FabPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let fab_small = SmallFloatingActionButton::new("fab-small", IconName::new("add")).build(cx);
        let fab_std = FloatingActionButton::new("fab-std", IconName::new("add"))
            .on_click(|_, window, cx| {
                show_snackbar(window, cx, Snackbar::new("Draft started"), None);
            })
            .build(cx);
        let fab_medium =
            MediumFloatingActionButton::new("fab-medium", IconName::new("add")).build(cx);
        let fab_large = LargeFloatingActionButton::new("fab-large", IconName::new("add")).build(cx);
        let fab_primary = FloatingActionButton::new("fab-primary", IconName::new("edit")).build(cx);
        let fab_secondary = FloatingActionButton::new("fab-secondary", IconName::new("edit"))
            .color(FabColor::Secondary)
            .build(cx);
        let fab_tertiary = FloatingActionButton::new("fab-tertiary", IconName::new("edit"))
            .color(FabColor::Tertiary)
            .build(cx);
        let fab_surface = FloatingActionButton::new("fab-surface", IconName::new("edit"))
            .color(FabColor::Surface)
            .build(cx);
        let fab_lowered = Fab::new("fab-lowered", IconName::new("star"))
            .lowered(true)
            .build(cx);

        cx.new(|_| Self {
            fab_small,
            fab_std,
            fab_medium,
            fab_large,
            fab_primary,
            fab_secondary,
            fab_tertiary,
            fab_surface,
            fab_lowered,
        })
    }
}

impl ComponentPage for FabPage {
    fn description(&self) -> &'static str {
        "The FAB represents the most important action on a screen. It puts key actions \
         within reach."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "FloatingActionButtonSample",
                description: "Floating action button examples",
            },
            ExampleMeta {
                name: "SmallFloatingActionButtonSample",
                description: "Floating action button examples",
            },
            ExampleMeta {
                name: "MediumFloatingActionButtonSample",
                description: "Floating action button examples",
            },
            ExampleMeta {
                name: "LargeFloatingActionButtonSample",
                description: "Floating action button examples",
            },
            ExampleMeta {
                name: "AnimatedFloatingActionButtonSample",
                description: "Floating action button examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => example_content([
                example_row(cx, "Standard", [self.fab_std.clone().into_any_element()]),
                example_row(cx, "Primary", [self.fab_primary.clone().into_any_element()]),
                example_row(
                    cx,
                    "Secondary",
                    [self.fab_secondary.clone().into_any_element()],
                ),
                example_row(
                    cx,
                    "Tertiary",
                    [self.fab_tertiary.clone().into_any_element()],
                ),
                example_row(cx, "Surface", [self.fab_surface.clone().into_any_element()]),
                example_row(cx, "Lowered", [self.fab_lowered.clone().into_any_element()]),
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
            4 => example_content(Vec::<AnyElement>::new()),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
