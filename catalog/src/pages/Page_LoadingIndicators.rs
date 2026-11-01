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

pub struct LoadingIndicatorsPage;

impl LoadingIndicatorsPage {
    pub fn new(_cx: &mut App) -> Entity<Self> {
        _cx.new(|_| Self)
    }
}

impl ComponentPage for LoadingIndicatorsPage {
    fn description(&self) -> &'static str {
        "Loading indicators express an unspecified wait time or display the length of \
         a loading process."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "LoadingIndicatorSample",
                description: "Loading indicators examples",
            },
            ExampleMeta {
                name: "ContainedLoadingIndicatorSample",
                description: "Loading indicators examples",
            },
            ExampleMeta {
                name: "DeterminateLoadingIndicatorSample",
                description: "Loading indicators examples",
            },
            ExampleMeta {
                name: "DeterminateContainedLoadingIndicatorSample",
                description: "Loading indicators examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => example_content([
                example_row(
                    cx,
                    "112 dp",
                    [div()
                        .size(px(112.))
                        .flex_none()
                        .child(
                            LoadingIndicator::new("loading-demo-default")
                                .size(px(112.))
                                .indicator_size(px(89.)),
                        )
                        .into_any_element()],
                ),
                example_row(
                    cx,
                    "48 dp",
                    [LoadingIndicator::new("loading-48")
                        .size(px(48.))
                        .into_any_element()],
                ),
                example_row(
                    cx,
                    "32 dp",
                    [LoadingIndicator::new("loading-32")
                        .size(px(32.))
                        .into_any_element()],
                ),
            ]),
            1 => example_content([
                example_row(
                    cx,
                    "112 dp",
                    [div()
                        .size(px(112.))
                        .flex_none()
                        .child(
                            ContainedLoadingIndicator::new("loading-demo-contained")
                                .size(px(112.))
                                .indicator_size(px(89.)),
                        )
                        .into_any_element()],
                ),
                example_row(
                    cx,
                    "48 dp",
                    [ContainedLoadingIndicator::new("contained-48")
                        .size(px(48.))
                        .into_any_element()],
                ),
            ]),
            2 => example_content([
                example_row(
                    cx,
                    "25 %",
                    [LoadingIndicator::new("determinate-25")
                        .progress(0.25)
                        .into_any_element()],
                ),
                example_row(
                    cx,
                    "60 %",
                    [LoadingIndicator::new("determinate-60")
                        .progress(0.6)
                        .into_any_element()],
                ),
                example_row(
                    cx,
                    "100 %",
                    [LoadingIndicator::new("determinate-100")
                        .progress(1.0)
                        .into_any_element()],
                ),
            ]),
            3 => example_content([example_row(
                cx,
                "Contained · 60 %",
                [LoadingIndicator::new("determinate-contained-60")
                    .contained()
                    .progress(0.6)
                    .into_any_element()],
            )]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
