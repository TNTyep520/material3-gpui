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

use gpui::{AnyElement, App, Entity, IntoElement, Render, Styled, Window, div, prelude::*, px};
use material3_gpui::icon::IconName;
use material3_gpui::prelude::*;

use super::{gallery, showcase_group};

pub struct AppBarsPage {
    fab: Entity<FabState>,
}

impl AppBarsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let fab = Fab::new("scaffold-fab", IconName::new("add")).build(cx);
        cx.new(|_| Self { fab })
    }
}

fn bar_row(bar: impl IntoElement, caption: &'static str, cx: &App) -> AnyElement {
    let typography = *cx.theme().typography();
    div()
        .w_full()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(px(8.))
        .child(
            typography
                .label_medium
                .apply(div())
                .text_color(cx.theme().colors().on_surface_variant)
                .child(caption),
        )
        .child(bar)
        .into_any_element()
}

impl Render for AppBarsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = *theme.colors();

        let bell_with_badge = div().child(badged(
            Icon::new(IconName::new("menu"))
                .size(px(24.))
                .color(colors.on_surface_variant),
            Badge::new("appbar-bell-badge").label("3"),
        ));

        gallery([
            showcase_group(
                cx,
                "Top app bar variants",
                [
                    bar_row(
                        TopAppBar::small("bar-small")
                            .title("Small")
                            .leading(
                                Icon::new(IconName::new("arrow_back"))
                                    .size(px(24.))
                                    .color(colors.on_surface),
                            )
                            .action(bell_with_badge)
                            .action(
                                Icon::new(IconName::new("more_vert"))
                                    .size(px(24.))
                                    .color(colors.on_surface_variant),
                            ),
                        "Small app bar",
                        cx,
                    ),
                    bar_row(
                        TopAppBar::medium("bar-medium")
                            .title("Medium")
                            .leading(
                                Icon::new(IconName::new("arrow_back"))
                                    .size(px(24.))
                                    .color(colors.on_surface),
                            )
                            .action(
                                Icon::new(IconName::new("more_vert"))
                                    .size(px(24.))
                                    .color(colors.on_surface_variant),
                            ),
                        "Medium app bar",
                        cx,
                    ),
                    bar_row(
                        TopAppBar::large("bar-large")
                            .title("Large")
                            .leading(
                                Icon::new(IconName::new("arrow_back"))
                                    .size(px(24.))
                                    .color(colors.on_surface),
                            )
                            .action(
                                Icon::new(IconName::new("more_vert"))
                                    .size(px(24.))
                                    .color(colors.on_surface_variant),
                            ),
                        "Large app bar",
                        cx,
                    ),
                ],
            ),
            showcase_group(
                cx,
                "Badges",
                [div()
                    .flex()
                    .items_center()
                    .gap(px(32.))
                    .child(badged(
                        Icon::new(IconName::new("menu"))
                            .size(px(24.))
                            .color(colors.on_surface_variant),
                        Badge::new("badge-bell-count").label("9"),
                    ))
                    .child(badged(
                        Icon::new(IconName::new("info"))
                            .size(px(24.))
                            .color(colors.on_surface_variant),
                        Badge::new("badge-inbox-dot"),
                    ))
                    .into_any_element()],
            ),
            showcase_group(
                cx,
                "Scaffold",
                [div()
                    .w_full()
                    .min_w_0()
                    .h(px(320.))
                    .overflow_hidden()
                    .rounded(px(24.))
                    .child(
                        Scaffold::new("scaffold-demo")
                            .top_bar(
                                TopAppBar::small("scaffold-bar")
                                    .title("Kiln report")
                                    .action(
                                        Icon::new(IconName::new("more_vert"))
                                            .size(px(24.))
                                            .color(colors.on_surface_variant),
                                    ),
                            )
                            .fab(self.fab.clone())
                            .child(
                                div()
                                    .p(px(16.))
                                    .text_color(colors.on_surface_variant)
                                    .child(
                                        "The FAB floats above the bottom-right corner and the \
                                     top bar stays pinned.",
                                    ),
                            ),
                    )
                    .into_any_element()],
            ),
        ])
    }
}
