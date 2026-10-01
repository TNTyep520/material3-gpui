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
    App, AppContext as _, Context, Entity, IntoElement, Render, WeakEntity, Window, div,
    prelude::*, px,
};
use material3_gpui::prelude::*;

use super::{LogErr as _, page, showcase_group};

pub struct CarouselPage {
    weak: WeakEntity<Self>,
    selected: usize,
}

impl CarouselPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            weak: cx.entity().downgrade(),
            selected: 1,
        })
    }
}

impl Render for CarouselPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = *theme.colors();
        let typography = *theme.typography();
        let weak = self.weak.clone();

        let hero = move |name: &'static str| {
            div()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap(px(8.))
                .child(
                    Icon::new(IconName::new("image"))
                        .size(px(28.))
                        .color(colors.on_surface_variant),
                )
                .child(typography.title_medium.apply(div()).child(name))
        };

        page(
            cx,
            "Carousel",
            "Multi-browse arrangement: the selected item expands, neighbors stay as previews.",
            [showcase_group(
                cx,
                "Multi-browse carousel",
                [div()
                    .w_full()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .child(
                        typography
                            .body_medium
                            .apply(div())
                            .text_color(colors.on_surface_variant)
                            .child(format!("Selected item {}", self.selected + 1)),
                    )
                    .child(
                        Carousel::new("carousel")
                            .selected(self.selected)
                            .on_select({
                                move |index, _, cx| {
                                    weak.update(cx, |page: &mut Self, cx: &mut Context<Self>| {
                                        page.selected = index;
                                        cx.notify();
                                    })
                                    .log_err();
                                }
                            })
                            .child(hero("Alpha"))
                            .child(hero("Bravo"))
                            .child(hero("Charlie"))
                            .child(hero("Delta"))
                            .child(hero("Echo")),
                    )
                    .into_any_element()],
            )],
        )
    }
}
