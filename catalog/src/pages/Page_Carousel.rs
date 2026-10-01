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

use gpui::{App, AppContext as _, Entity, IntoElement, Render, Window, div, prelude::*, px};
use material3_gpui::prelude::*;

use super::{page, showcase_group};

pub struct CarouselPage {
    carousel: Entity<CarouselState>,
}

impl CarouselPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let hero_colors = *cx.theme().colors();
        let hero_typography = *cx.theme().typography();
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
                        .color(hero_colors.on_surface_variant),
                )
                .child(hero_typography.title_medium.apply(div()).child(name))
                .into_any_element()
        };
        let carousel = Carousel::new("carousel")
            .selected(1)
            .item(move || hero("Alpha"))
            .item(move || hero("Bravo"))
            .item(move || hero("Charlie"))
            .item(move || hero("Delta"))
            .item(move || hero("Echo"))
            .build(cx);
        cx.new(|cx| {
            cx.observe(&carousel, |_, _, cx| cx.notify()).detach();
            Self { carousel }
        })
    }
}

impl Render for CarouselPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = *theme.colors();
        let typography = *theme.typography();
        let carousel = self.carousel.clone();

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
                            .child(format!(
                                "Selected item {}",
                                self.carousel.read(cx).selected() + 1
                            )),
                    )
                    .child(carousel)
                    .into_any_element()],
            )],
        )
    }
}
