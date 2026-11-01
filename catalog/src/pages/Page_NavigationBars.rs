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

pub struct NavigationBarsPage {
    nav_bar: Entity<NavigationBarState>,
    nav_bar_short: Entity<NavigationBarState>,
    nav_bar_badge: Entity<NavigationBarState>,
    scaffold_bar: Entity<NavigationBarState>,
}

impl NavigationBarsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let nav_bar = NavigationBar::new("nav-bar")
            .items([
                NavigationItemSpec::new("Home", IconName::new("home")),
                NavigationItemSpec::new("Search", IconName::new("search")),
                NavigationItemSpec::new("Profile", IconName::new("person")),
                NavigationItemSpec::new("Settings", IconName::new("settings")),
            ])
            .selected(0)
            .build(cx);
        let nav_bar_short = NavigationBar::new("nav-bar-short")
            .items([
                NavigationItemSpec::new("Home", IconName::new("home")),
                NavigationItemSpec::new("Profile", IconName::new("person")),
            ])
            .selected(0)
            .build(cx);
        let nav_bar_badge = NavigationBar::new("nav-bar-badge")
            .item(NavigationItemSpec::new("Inbox", IconName::new("info")).badge("24"))
            .item(NavigationItemSpec::new("Starred", IconName::new("star")))
            .item(NavigationItemSpec::new("Sent", IconName::new("edit")))
            .selected(0)
            .build(cx);
        let scaffold_bar = NavigationBar::new("scaffold-nav-bar-bar")
            .items([
                NavigationItemSpec::new("Home", IconName::new("home")),
                NavigationItemSpec::new("Inbox", IconName::new("info")),
                NavigationItemSpec::new("Profile", IconName::new("person")),
            ])
            .selected(0)
            .build(cx);
        cx.new(|_| Self {
            nav_bar,
            nav_bar_short,
            nav_bar_badge,
            scaffold_bar,
        })
    }
}

impl ComponentPage for NavigationBarsPage {
    fn description(&self) -> &'static str {
        "Navigation bars offer a persistent and convenient way to switch between \
         primary destinations in an app."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "NavigationBarSample",
                description: "Navigation bar examples",
            },
            ExampleMeta {
                name: "ShortNavigationBarSample",
                description: "Navigation bar examples",
            },
            ExampleMeta {
                name: "NavigationBarItemWithBadge",
                description: "Badge examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => {
                let colors = *cx.theme().colors();
                example_content([
                    example_row(
                        cx,
                        "Navigation bar",
                        [div()
                            .w_full()
                            .min_w_0()
                            .child(self.nav_bar.clone())
                            .into_any_element()],
                    ),
                    example_row(
                        cx,
                        "Scaffold with navigation bar",
                        [div()
                            .w_full()
                            .min_w_0()
                            .h(px(280.))
                            .overflow_hidden()
                            .rounded(px(24.))
                            .child(
                                Scaffold::new("scaffold-nav-bar")
                                    .bottom_bar(self.scaffold_bar.clone())
                                    .child(
                                        div()
                                            .p(px(16.))
                                            .text_color(colors.on_surface_variant)
                                            .child(
                                                "The navigation bar sits at the bottom of the \
                                                 scaffold.",
                                            ),
                                    ),
                            )
                            .into_any_element()],
                    ),
                ])
            }
            1 => example_content([example_row(
                cx,
                "Short navigation bar",
                [div()
                    .w_full()
                    .min_w_0()
                    .child(self.nav_bar_short.clone())
                    .into_any_element()],
            )]),
            2 => example_content([example_row(
                cx,
                "Item with badge",
                [div()
                    .w_full()
                    .min_w_0()
                    .child(self.nav_bar_badge.clone())
                    .into_any_element()],
            )]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
