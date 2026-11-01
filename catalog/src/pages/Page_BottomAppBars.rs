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

use gpui::{
    AnyElement, App, AppContext as _, Entity, IntoElement, ParentElement as _, Styled, div, px,
};

use material3_gpui::prelude::*;

use super::{ComponentPage, ExampleMeta, example_content, example_row};

pub struct BottomAppBarsPage {
    fab: Entity<FabState>,
    scaffold_fab: Entity<FabState>,
}

impl BottomAppBarsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let fab = Fab::new("bottom-bar-fab", IconName::new("add")).build(cx);
        let scaffold_fab = Fab::new("scaffold-bottom-fab", IconName::new("add")).build(cx);
        cx.new(|_| Self { fab, scaffold_fab })
    }
}

fn bottom_app_bar(id: &'static str, fab: Option<impl IntoElement>, cx: &App) -> AnyElement {
    let colors = *cx.theme().colors();
    let mut bar = BottomAppBar::new(id)
        .action(
            Icon::new(IconName::new("menu"))
                .size(px(24.))
                .color(colors.on_surface),
        )
        .action(
            Icon::new(IconName::new("search"))
                .size(px(24.))
                .color(colors.on_surface),
        );
    if let Some(fab) = fab {
        bar = bar.floating_action_button(fab);
    }
    bar.into_any_element()
}

impl ComponentPage for BottomAppBarsPage {
    fn description(&self) -> &'static str {
        "A bottom app bar displays navigation and key actions at the bottom of mobile \
         screens."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "SimpleBottomAppBar",
                description: "Bottom app bar examples",
            },
            ExampleMeta {
                name: "BottomAppBarWithFAB",
                description: "Bottom app bar examples",
            },
            ExampleMeta {
                name: "BottomAppBarWithOverflow",
                description: "Bottom app bar examples",
            },
            ExampleMeta {
                name: "ExitAlwaysBottomAppBar",
                description: "Bottom app bar examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => example_content([example_row(
                cx,
                "Plain",
                [bottom_app_bar(
                    "bottom-bar-plain",
                    Option::<AnyElement>::None,
                    cx,
                )],
            )]),
            1 => {
                let colors = *cx.theme().colors();
                example_content([
                    example_row(
                        cx,
                        "With FAB",
                        [bottom_app_bar(
                            "bottom-bar-fab-row",
                            Some(self.fab.clone()),
                            cx,
                        )],
                    ),
                    example_row(
                        cx,
                        "Scaffold",
                        [div()
                            .w_full()
                            .min_w_0()
                            .h(px(320.))
                            .overflow_hidden()
                            .rounded(px(24.))
                            .child(
                                Scaffold::new("scaffold-bottom-bar")
                                    .bottom_bar(
                                        BottomAppBar::new("scaffold-bottom-bar")
                                            .action(
                                                Icon::new(IconName::new("menu"))
                                                    .size(px(24.))
                                                    .color(colors.on_surface),
                                            )
                                            .floating_action_button(self.scaffold_fab.clone()),
                                    )
                                    .child(
                                        div()
                                            .p(px(16.))
                                            .text_color(colors.on_surface_variant)
                                            .child(
                                                "The bottom app bar stays pinned and the \
                                                 FAB docks above it.",
                                            ),
                                    ),
                            )
                            .into_any_element()],
                    ),
                ])
            }
            2 => {
                let colors = *cx.theme().colors();
                example_content([example_row(
                    cx,
                    "Flex box",
                    [BottomAppBar::new("bottom-bar-overflow")
                        .action(
                            Icon::new(IconName::new("edit"))
                                .size(px(24.))
                                .color(colors.on_surface),
                        )
                        .action(
                            Icon::new(IconName::new("more_vert"))
                                .size(px(24.))
                                .color(colors.on_surface),
                        )
                        .into_any_element()],
                )])
            }
            3 => {
                let colors = *cx.theme().colors();
                example_content([example_row(
                    cx,
                    "Flexible",
                    [FlexibleBottomAppBar::new("bottom-bar-flexible")
                        .action(
                            Icon::new(IconName::new("menu"))
                                .size(px(24.))
                                .color(colors.on_surface),
                        )
                        .action(
                            Icon::new(IconName::new("search"))
                                .size(px(24.))
                                .color(colors.on_surface),
                        )
                        .into_any_element()],
                )])
            }
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
