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

use super::{ComponentPage, ExampleMeta, example_content};

const LIBRARY: [(&str, &str, &str); 3] = [
    (
        "Home recordings",
        "Studio takes from the milk crate: 84 items, 3.2 GB.",
        "home",
    ),
    (
        "Field notes",
        "Scratchy zoom mics and wind from the coast: 37 items.",
        "search",
    ),
    (
        "Releases",
        "Everything pressed, posted or shipped: 12 items.",
        "person",
    ),
];

pub struct TabsPage {
    tabbar: Entity<TabBarState>,
    secondary: Entity<TabBarState>,
    scrollable: Entity<TabBarState>,
    leading_icon: Entity<TabBarState>,
}

impl TabsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let tabbar = TabBar::new("tabs")
            .tab(Tab::new("Home").icon(IconName::new("home")))
            .tab(Tab::new("Search").icon(IconName::new("search")))
            .tab(Tab::new("Profile").icon(IconName::new("person")))
            .selected(0)
            .build(cx);
        let secondary = TabBar::new("tabs-secondary")
            .variant(TabRowVariant::Secondary)
            .tab(Tab::new("Songs"))
            .tab(Tab::new("Albums"))
            .tab(Tab::new("Playlists"))
            .selected(0)
            .build(cx);
        let scrollable = TabBar::new("tabs-scrollable")
            .scrollable(true)
            .tabs([
                Tab::new("Chapter 1"),
                Tab::new("Chapter 2"),
                Tab::new("Chapter 3"),
                Tab::new("Chapter 4"),
                Tab::new("Chapter 5"),
                Tab::new("Chapter 6"),
                Tab::new("Chapter 7"),
            ])
            .selected(0)
            .build(cx);
        let leading_icon = TabBar::new("tabs-leading-icon")
            .tab(LeadingIconTab::new("Recent", IconName::new("history")))
            .tab(LeadingIconTab::new("Starred", IconName::new("star")))
            .selected(0)
            .build(cx);
        cx.new(|cx| {
            cx.observe(&tabbar, |_, _, cx| cx.notify()).detach();
            Self {
                tabbar,
                secondary,
                scrollable,
                leading_icon,
            }
        })
    }
}

fn tab_content_row(selected_tab: usize, cx: &App) -> AnyElement {
    let theme = cx.theme();
    let colors = *theme.colors();
    let typography = *theme.typography();
    let (title, caption, icon) = LIBRARY[selected_tab.min(LIBRARY.len() - 1)];
    let icon = IconName::new(icon);
    div()
        .pt(px(16.))
        .child(
            div()
                .w_full()
                .min_w_0()
                .p(px(16.))
                .flex()
                .items_center()
                .gap(px(16.))
                .child(
                    div()
                        .size(px(48.))
                        .rounded_full()
                        .bg(colors.secondary_container)
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            Icon::new(icon)
                                .size(px(24.))
                                .color(colors.on_secondary_container),
                        ),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .gap(px(2.))
                        .child(
                            typography
                                .title_medium
                                .apply(div())
                                .text_color(colors.on_surface)
                                .child(title),
                        )
                        .child(
                            typography
                                .body_small
                                .apply(div())
                                .text_color(colors.on_surface_variant)
                                .child(caption),
                        ),
                ),
        )
        .into_any_element()
}

impl ComponentPage for TabsPage {
    fn description(&self) -> &'static str {
        "Tabs organize content across different screens, data sets, and other interactions."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "PrimaryIconTabs",
                description: "Tabs examples",
            },
            ExampleMeta {
                name: "SecondaryTextTabs",
                description: "Tabs examples",
            },
            ExampleMeta {
                name: "ScrollingPrimaryTextTabs",
                description: "Tabs examples",
            },
            ExampleMeta {
                name: "LeadingIconTabs",
                description: "Tabs examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => example_content([div()
                .flex()
                .flex_col()
                .w_full()
                .min_w_0()
                .child(div().w_full().child(self.tabbar.clone()))
                .child(tab_content_row(self.tabbar.read(cx).selected(), cx))
                .into_any_element()]),
            1 => example_content([div()
                .w_full()
                .min_w_0()
                .child(self.secondary.clone())
                .into_any_element()]),
            2 => example_content([div()
                .w_full()
                .min_w_0()
                .child(self.scrollable.clone())
                .into_any_element()]),
            3 => example_content([div()
                .w_full()
                .min_w_0()
                .child(self.leading_icon.clone())
                .into_any_element()]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
