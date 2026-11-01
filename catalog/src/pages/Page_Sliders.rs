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
// 参考 Compose Material Catalog SlidersSamples.kt 的示例分组。
use gpui::{
    AnyElement, App, AppContext as _, Entity, IntoElement, ParentElement as _, Styled, div, px,
};

use material3_gpui::prelude::*;

use super::{ComponentPage, ExampleMeta, example_content, example_row};

pub struct SlidersPage {
    continuous: Entity<SliderState>,
    continuous_disabled: Entity<SliderState>,
    discrete_10: Entity<SliderState>,
    discrete_5: Entity<SliderState>,
    centered_negative: Entity<SliderState>,
    centered_zero: Entity<SliderState>,
    centered_positive: Entity<SliderState>,
    sizes: [Entity<SliderState>; 5],
    value_indicator: Entity<SliderState>,
    track_icons: Entity<SliderState>,
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
        let continuous = Slider::new(0., 100., 24.).build(cx);
        let continuous_disabled = Slider::new(0., 100., 50.).disabled(true).build(cx);
        let discrete_10 = Slider::new(0., 100., 30.).step(10.).build(cx);
        let discrete_5 = Slider::new(0., 100., 70.).step(5.).build(cx);
        let centered_negative = Slider::new(-100., 100., -45.).centered(true).build(cx);
        let centered_zero = Slider::new(-100., 100., 0.).centered(true).build(cx);
        let centered_positive = Slider::new(-100., 100., 60.)
            .step(20.)
            .centered(true)
            .build(cx);
        let value_indicator = Slider::new(0., 100., 50.)
            .step(10.)
            .show_value_indicator(true)
            .build(cx);
        let track_icons = Slider::new(0., 100., 40.)
            .active_track_icon(IconName::new("visibility"))
            .inactive_track_icon(IconName::new("visibility"))
            .build(cx);
        cx.new(|_| Self {
            continuous,
            continuous_disabled,
            discrete_10,
            discrete_5,
            centered_negative,
            centered_zero,
            centered_positive,
            sizes,
            value_indicator,
            track_icons,
        })
    }
}

/// 固定宽度包裹滑块,保持各行布局与旧示例一致。
fn slider_at(slider: impl IntoElement, width: f32) -> AnyElement {
    div().w(px(width)).child(slider).into_any_element()
}

impl ComponentPage for SlidersPage {
    fn description(&self) -> &'static str {
        "Sliders allow users to make selections from a range of values."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "SliderSample",
                description: "Sliders examples",
            },
            ExampleMeta {
                name: "StepsSliderSample",
                description: "Sliders examples",
            },
            ExampleMeta {
                name: "CenteredSliderSample",
                description: "Sliders examples",
            },
            ExampleMeta {
                name: "RangeSliderSample",
                description: "Sliders examples",
            },
            ExampleMeta {
                name: "SliderWithCustomThumbSample",
                description: "Sliders examples",
            },
            ExampleMeta {
                name: "SliderWithTrackIconsSample",
                description: "Sliders examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => {
                let size_labels = [
                    "Extra small",
                    "Small",
                    "Medium",
                    "Large · track icons",
                    "Extra large",
                ];
                example_content(
                    [
                        example_row(cx, "Continuous", [slider_at(self.continuous.clone(), 260.)]),
                        example_row(
                            cx,
                            "Disabled",
                            [slider_at(self.continuous_disabled.clone(), 260.)],
                        ),
                    ]
                    .into_iter()
                    .chain(self.sizes.iter().zip(size_labels).map(|(slider, label)| {
                        example_row(cx, label, [slider_at(slider.clone(), 320.)])
                    }))
                    .collect::<Vec<_>>(),
                )
            }
            1 => example_content([example_row(
                cx,
                "Discrete",
                [
                    slider_at(self.discrete_10.clone(), 260.),
                    slider_at(self.discrete_5.clone(), 260.),
                ],
            )]),
            2 => example_content([
                example_row(
                    cx,
                    "Negative",
                    [slider_at(self.centered_negative.clone(), 260.)],
                ),
                example_row(cx, "Zero", [slider_at(self.centered_zero.clone(), 260.)]),
                example_row(
                    cx,
                    "Positive · steps",
                    [slider_at(self.centered_positive.clone(), 260.)],
                ),
            ]),
            3 => example_content([example_row(
                cx,
                "Range",
                [
                    slider_at(RangeSlider::new("range-slider-1", 0.2, 0.8), 260.),
                    slider_at(RangeSlider::new("range-slider-2", 0.1, 0.5), 260.),
                    slider_at(
                        RangeSlider::new("range-slider-3", 0.3, 0.7).enabled(false),
                        260.,
                    ),
                ],
            )]),
            4 => example_content([example_row(
                cx,
                "Value indicator",
                [slider_at(self.value_indicator.clone(), 260.)],
            )]),
            5 => example_content([example_row(
                cx,
                "Track icons",
                [slider_at(self.track_icons.clone(), 260.)],
            )]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
