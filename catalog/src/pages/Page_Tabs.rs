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

const LIBRARY: [(&str, &str, IconName); 3] = [
    (
        "Home recordings",
        "Studio takes from the milk crate: 84 items, 3.2 GB.",
        IconName::Home,
    ),
    (
        "Field notes",
        "Scratchy zoom mics and wind from the coast: 37 items.",
        IconName::Search,
    ),
    (
        "Releases",
        "Everything pressed, posted or shipped: 12 items.",
        IconName::Person,
    ),
];

pub struct TabsPage {
    tabbar: Entity<TabBarState>,
}

impl TabsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let tabbar = TabBar::new("tabs")
            .tab(Tab::new("Home").icon(IconName::Home))
            .tab(Tab::new("Search").icon(IconName::Search))
            .tab(Tab::new("Profile").icon(IconName::Person))
            .selected(0)
            .build(cx);
        cx.new(|cx| {
            cx.observe(&tabbar, |_, _, cx| cx.notify()).detach();
            Self { tabbar }
        })
    }
}

impl Render for TabsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = *theme.colors();
        let typography = *theme.typography();
        let selected_tab = self.tabbar.read(cx).selected();
        let (title, caption, icon) = LIBRARY[selected_tab.min(LIBRARY.len() - 1)];

        gallery([showcase_group(
            cx,
            "Primary tabs",
            [div()
                .flex()
                .flex_col()
                .w_full()
                .min_w_0()
                .child(div().w_full().child(self.tabbar.clone()))
                .child(
                    div().pt(px(16.)).child(
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
                    ),
                )
                .into_any_element()],
        )])
    }
}
