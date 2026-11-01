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

pub struct NavigationRailsPage {
    rail: Entity<NavigationRailState>,
    compact_rail: Entity<NavigationRailState>,
    wide_rail_children: Vec<Entity<ButtonState>>,
    wide_rail_expanded_children: Vec<Entity<ButtonState>>,
}

impl NavigationRailsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let rail = NavigationRail::new("nav-rail")
            .header(
                Fab::new("rail-fab", IconName::new("add"))
                    .size(FabSize::Small)
                    .build(cx),
            )
            .items([
                NavigationItemSpec::new("Inbox", IconName::new("info")),
                NavigationItemSpec::new("Starred", IconName::new("star")),
                NavigationItemSpec::new("Sent", IconName::new("edit")),
            ])
            .selected(0)
            .build(cx);
        let compact_rail = NavigationRail::new("nav-rail-compact")
            .items([
                NavigationItemSpec::new("Home", IconName::new("home")),
                NavigationItemSpec::new("Search", IconName::new("search")),
            ])
            .selected(0)
            .build(cx);
        let wide_rail_children = vec![
            Button::new("wide-rail-home", "Home").text().build(cx),
            Button::new("wide-rail-settings", "Settings")
                .text()
                .build(cx),
        ];
        let wide_rail_expanded_children = vec![
            Button::new("wide-rail-expanded-home", "Home")
                .text()
                .leading_icon(IconName::new("home"))
                .build(cx),
            Button::new("wide-rail-expanded-settings", "Settings")
                .text()
                .leading_icon(IconName::new("settings"))
                .build(cx),
        ];
        cx.new(|_| Self {
            rail,
            compact_rail,
            wide_rail_children,
            wide_rail_expanded_children,
        })
    }
}

impl ComponentPage for NavigationRailsPage {
    fn description(&self) -> &'static str {
        "Navigation rails provide access to primary destinations in apps when using \
         tablet and desktop screens."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "NavigationRailSample",
                description: "Navigation rail examples",
            },
            ExampleMeta {
                name: "NavigationRailBottomAlignSample",
                description: "Navigation rail examples",
            },
            ExampleMeta {
                name: "WideNavigationRailCollapsedSample",
                description: "Navigation rail examples",
            },
            ExampleMeta {
                name: "WideNavigationRailExpandedSample",
                description: "Navigation rail examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => example_content([example_row(
                cx,
                "Rail",
                [div()
                    .h(px(360.))
                    .flex_none()
                    .child(self.rail.clone())
                    .into_any_element()],
            )]),
            1 => example_content([example_row(
                cx,
                "Compact rail",
                [div()
                    .h(px(320.))
                    .flex_none()
                    .child(self.compact_rail.clone())
                    .into_any_element()],
            )]),
            2 => example_content([example_row(
                cx,
                "Wide rail",
                [div()
                    .h(px(320.))
                    .flex_none()
                    .overflow_hidden()
                    .child(
                        WideNavigationRail::new("wide-rail")
                            .children(self.wide_rail_children.clone()),
                    )
                    .into_any_element()],
            )]),
            3 => example_content([example_row(
                cx,
                "Wide rail expanded",
                [div()
                    .h(px(320.))
                    .flex_none()
                    .overflow_hidden()
                    .child(
                        WideNavigationRail::new("wide-rail-expanded")
                            .children(self.wide_rail_expanded_children.clone()),
                    )
                    .into_any_element()],
            )]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
