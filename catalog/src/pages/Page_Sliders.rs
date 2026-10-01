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
// 参考 https://github.com/Glavo/m3fx/blob/main/demo/src/main/java/org/glavo/m3fx/demo/SlidersDemoPage.java

use gpui::{
    App, AppContext as _, Entity, IntoElement, Render, Styled, Window, div, prelude::*, px,
};
use material3_gpui::prelude::*;

use super::{full_width_group, page, showcase_group};

pub struct SlidersPage {
    continuous_a: Entity<SliderState>,
    continuous_b: Entity<SliderState>,
    continuous_disabled: Entity<SliderState>,
    discrete_10: Entity<SliderState>,
    discrete_5: Entity<SliderState>,
    centered_negative: Entity<SliderState>,
    centered_zero: Entity<SliderState>,
    centered_positive: Entity<SliderState>,
    sizes: [Entity<SliderState>; 5],
    value_indicator: Entity<SliderState>,
    vertical: Entity<SliderState>,
    vertical_centered: Entity<SliderState>,
}

impl SlidersPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let sizes = [
            Slider::new(0., 100., 50.)
                .size(SliderSize::ExtraSmall)
                .build(cx),
            Slider::new(0., 100., 50.).size(SliderSize::Small).build(cx),
            Slider::new(0., 100., 50.)
                .size(SliderSize::Medium)
                .build(cx),
            Slider::new(0., 100., 50.)
                .size(SliderSize::Large)
                .active_track_icon(IconName::new("visibility"))
                .inactive_track_icon(IconName::new("visibility"))
                .build(cx),
            Slider::new(0., 100., 50.)
                .size(SliderSize::ExtraLarge)
                .active_track_icon(IconName::new("visibility"))
                .inactive_track_icon(IconName::new("visibility"))
                .build(cx),
        ];
        cx.new(|cx| Self {
            continuous_a: Slider::new(0., 100., 24.).build(cx),
            continuous_b: Slider::new(0., 100., 64.).build(cx),
            continuous_disabled: Slider::new(0., 100., 50.).disabled(true).build(cx),
            discrete_10: Slider::new(0., 100., 30.).step(10.).build(cx),
            discrete_5: Slider::new(0., 100., 70.).step(5.).build(cx),
            centered_negative: Slider::new(-100., 100., -45.).centered(true).build(cx),
            centered_zero: Slider::new(-100., 100., 0.).centered(true).build(cx),
            centered_positive: Slider::new(-100., 100., 60.)
                .step(20.)
                .centered(true)
                .build(cx),
            sizes,
            value_indicator: Slider::new(0., 100., 50.)
                .step(10.)
                .show_value_indicator(true)
                .build(cx),
            vertical: VerticalSlider::new(0., 100., 40.).build(cx),
            vertical_centered: VerticalSlider::new(-100., 100., -40.)
                .centered(true)
                .build(cx),
        })
    }
}

fn slider_row(cx: &App, label: &'static str, slider: impl IntoElement) -> impl IntoElement {
    let theme = cx.theme();
    div()
        .w_full()
        .min_w_0()
        .flex()
        .items_center()
        .gap(px(16.))
        .child(
            theme
                .typography()
                .label_large
                .apply(div())
                .w(px(88.))
                .flex_none()
                .text_color(theme.colors().on_surface_variant)
                .child(label),
        )
        .child(div().flex_1().min_w_0().child(slider))
}

impl Render for SlidersPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let size_labels = [
            "XS · 16 dp",
            "S · 24 dp",
            "M · 40 dp",
            "L · 56 dp",
            "XL · 96 dp",
        ];

        page(
            cx,
            "Sliders",
            "Continuous, discrete, centered, range and expressive sizes.",
            [
                showcase_group(
                    cx,
                    "Continuous",
                    [
                        div()
                            .w(px(260.))
                            .max_w_full()
                            .child(self.continuous_a.clone()),
                        div()
                            .w(px(260.))
                            .max_w_full()
                            .child(self.continuous_b.clone()),
                        div()
                            .w(px(260.))
                            .max_w_full()
                            .child(self.continuous_disabled.clone()),
                    ],
                )
                .into_any_element(),
                showcase_group(
                    cx,
                    "Discrete",
                    [
                        div()
                            .w(px(260.))
                            .max_w_full()
                            .child(self.discrete_10.clone()),
                        div()
                            .w(px(260.))
                            .max_w_full()
                            .child(self.discrete_5.clone()),
                    ],
                )
                .into_any_element(),
                showcase_group(
                    cx,
                    "Centered",
                    [
                        div()
                            .w(px(260.))
                            .max_w_full()
                            .child(self.centered_negative.clone()),
                        div()
                            .w(px(260.))
                            .max_w_full()
                            .child(self.centered_zero.clone()),
                        div()
                            .w(px(260.))
                            .max_w_full()
                            .child(self.centered_positive.clone()),
                    ],
                )
                .into_any_element(),
                showcase_group(
                    cx,
                    "Range",
                    [
                        div().w(px(300.)).max_w_full().child(RangeSlider::new(
                            "range-continuous",
                            0.20,
                            0.78,
                        )),
                        div().w(px(300.)).max_w_full().child(
                            RangeSlider::new("range-small", 0.30, 0.70).size(SliderSize::Small),
                        ),
                        div()
                            .w(px(300.))
                            .max_w_full()
                            .child(RangeSlider::new("range-disabled", 0.35, 0.85).enabled(false)),
                    ],
                )
                .into_any_element(),
                full_width_group(
                    cx,
                    "Expressive sizes",
                    self.sizes
                        .iter()
                        .zip(size_labels)
                        .map(|(slider, label)| slider_row(cx, label, slider.clone()))
                        .collect::<Vec<_>>(),
                )
                .into_any_element(),
                showcase_group(
                    cx,
                    "Value indicator",
                    [div()
                        .w(px(260.))
                        .max_w_full()
                        .child(self.value_indicator.clone())],
                )
                .into_any_element(),
                showcase_group(
                    cx,
                    "Vertical",
                    [
                        div().h(px(220.)).flex_none().child(self.vertical.clone()),
                        div()
                            .h(px(220.))
                            .flex_none()
                            .child(self.vertical_centered.clone()),
                    ],
                )
                .into_any_element(),
            ],
        )
    }
}
