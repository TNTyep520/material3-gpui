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
use gpui::{AnyElement, App, AppContext as _, Entity, IntoElement, px};

use material3_gpui::prelude::*;

use super::{ComponentPage, ExampleMeta, example_content, example_row};

/// 为顶部应用栏追加标准 action 集;各变体包装类型均提供同名 action builder。
macro_rules! finish_bar {
    ($bar:expr, $actions:expr) => {{
        let mut bar = $bar;
        for action in $actions {
            bar = bar.action(action);
        }
        bar
    }};
}

pub struct TopAppBarsPage;

impl TopAppBarsPage {
    pub fn new(_cx: &mut App) -> Entity<Self> {
        _cx.new(|_| Self)
    }
}

impl ComponentPage for TopAppBarsPage {
    fn description(&self) -> &'static str {
        "Top app bars display information and actions at the top of a screen."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "SimpleTopAppBar",
                description: "Top app bar examples",
            },
            ExampleMeta {
                name: "SimpleCenterAlignedTopAppBar",
                description: "Top app bar examples",
            },
            ExampleMeta {
                name: "ExitUntilCollapsedMediumTopAppBar",
                description: "Top app bar examples",
            },
            ExampleMeta {
                name: "ExitUntilCollapsedLargeTopAppBar",
                description: "Top app bar examples",
            },
            ExampleMeta {
                name: "ExitUntilCollapsedMediumFlexibleTopAppBar",
                description: "Top app bar examples",
            },
            ExampleMeta {
                name: "ExitUntilCollapsedLargeFlexibleTopAppBar",
                description: "Top app bar examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        let colors = *cx.theme().colors();
        let nav_icon = || {
            Icon::new(IconName::new("arrow_back"))
                .size(px(24.))
                .color(colors.on_surface)
        };
        let actions = || {
            [
                Icon::new(IconName::new("favorite"))
                    .size(px(24.))
                    .color(colors.on_surface_variant)
                    .into_any_element(),
                Icon::new(IconName::new("more_vert"))
                    .size(px(24.))
                    .color(colors.on_surface_variant)
                    .into_any_element(),
            ]
        };
        match index {
            0 => example_content([example_row(
                cx,
                "Small",
                [finish_bar!(
                    TopAppBar::small("bar-small")
                        .title("Simple top app bar")
                        .navigation_icon(nav_icon()),
                    actions()
                )],
            )]),
            1 => example_content([example_row(
                cx,
                "Center aligned",
                [CenterAlignedTopAppBar::new("bar-center")
                    .title("Center aligned")
                    .navigation_icon(nav_icon())
                    .action(
                        Icon::new(IconName::new("more_vert"))
                            .size(px(24.))
                            .color(colors.on_surface_variant),
                    )],
            )]),
            2 => example_content([example_row(
                cx,
                "Medium",
                [finish_bar!(
                    MediumTopAppBar::new("bar-medium")
                        .title("Medium top app bar")
                        .navigation_icon(nav_icon()),
                    actions()
                )],
            )]),
            3 => example_content([example_row(
                cx,
                "Large",
                [finish_bar!(
                    LargeTopAppBar::new("bar-large")
                        .title("Large top app bar")
                        .navigation_icon(nav_icon()),
                    actions()
                )],
            )]),
            4 => example_content([example_row(
                cx,
                "Medium flexible",
                [finish_bar!(
                    MediumFlexibleTopAppBar::new("bar-medium-flexible")
                        .title("Medium flexible")
                        .navigation_icon(nav_icon()),
                    actions()
                )],
            )]),
            5 => example_content([example_row(
                cx,
                "Large flexible",
                [finish_bar!(
                    LargeFlexibleTopAppBar::new("bar-large-flexible")
                        .title("Large flexible")
                        .navigation_icon(nav_icon()),
                    actions()
                )],
            )]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
