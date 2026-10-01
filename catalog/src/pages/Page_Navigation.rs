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

use gpui::{App, Entity, IntoElement, Render, Styled, Window, div, prelude::*, px};
use material3_gpui::prelude::*;

use super::{gallery, showcase_group};

pub struct NavigationPage {
    nav_bar: Entity<NavigationBarState>,
    nav_rail: Entity<NavigationRailState>,
    nav_drawer: Entity<NavigationDrawerState>,
}

impl NavigationPage {
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
        let nav_rail = NavigationRail::new("nav-rail")
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
        let nav_drawer = NavigationDrawer::new("nav-drawer")
            .item(NavigationItemSpec::new("Inbox", IconName::new("info")).badge("24"))
            .item(NavigationItemSpec::new("Starred", IconName::new("star")))
            .section("Labels")
            .item(NavigationItemSpec::new("Work", IconName::new("edit")))
            .item(NavigationItemSpec::new("Personal", IconName::new("person")))
            .selected(0)
            .build(cx);

        cx.new(|_| Self {
            nav_bar,
            nav_rail,
            nav_drawer,
        })
    }
}

fn icon_strip(cx: &App) -> impl IntoElement {
    let color = cx.theme().colors().on_surface_variant;
    div()
        .flex()
        .flex_wrap()
        .items_center()
        .gap(px(16.))
        .children(
            [
                IconName::new("home"),
                IconName::new("search"),
                IconName::new("settings"),
                IconName::new("favorite"),
                IconName::new("star"),
                IconName::new("person"),
                IconName::new("edit"),
                IconName::new("delete"),
                IconName::new("info"),
                IconName::new("menu"),
                IconName::new("more_vert"),
                IconName::new("check"),
                IconName::new("close"),
                IconName::new("add"),
                IconName::new("arrow_back"),
                IconName::new("chevron_right"),
                IconName::new("progress_activity"),
            ]
            .into_iter()
            .map(|name| Icon::new(name).size(px(24.)).color(color)),
        )
}

fn commute_strip(cx: &App) -> impl IntoElement {
    let color = cx.theme().colors().on_surface_variant;
    div()
        .flex()
        .flex_wrap()
        .items_center()
        .gap(px(16.))
        .children([
            Icon::new(IconName::new("directions_walk"))
                .size(px(24.))
                .color(color),
            Icon::new(IconName::new("directions_bike"))
                .size(px(24.))
                .color(color),
            Icon::new(IconName::new("directions_car"))
                .size(px(24.))
                .color(color),
        ])
}

impl Render for NavigationPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        gallery([
            showcase_group(
                cx,
                "Glyph set",
                [
                    icon_strip(cx).into_any_element(),
                    commute_strip(cx).into_any_element(),
                ],
            ),
            showcase_group(
                cx,
                "Navigation bar",
                [div()
                    .w_full()
                    .min_w_0()
                    .child(self.nav_bar.clone())
                    .into_any_element()],
            ),
            showcase_group(
                cx,
                "Rail & drawer",
                [
                    div()
                        .h(px(360.))
                        .child(self.nav_rail.clone())
                        .into_any_element(),
                    div()
                        .w(px(360.))
                        .max_w_full()
                        .h(px(360.))
                        .overflow_hidden()
                        .child(self.nav_drawer.clone())
                        .into_any_element(),
                ],
            ),
        ])
    }
}
