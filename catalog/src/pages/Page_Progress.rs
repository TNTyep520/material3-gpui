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

pub struct ProgressPage;

impl ProgressPage {
    pub fn new(_cx: &mut App) -> Entity<Self> {
        _cx.new(|_| Self)
    }
}

impl ComponentPage for ProgressPage {
    fn description(&self) -> &'static str {
        "Progress indicators express an unspecified wait time or display the length of \
         a process."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "LinearProgressIndicatorSample",
                description: "Progress indicators examples",
            },
            ExampleMeta {
                name: "LinearWavyProgressIndicatorSample",
                description: "Progress indicators examples",
            },
            ExampleMeta {
                name: "IndeterminateLinearProgressIndicatorSample",
                description: "Progress indicators examples",
            },
            ExampleMeta {
                name: "IndeterminateLinearWavyProgressIndicatorSample",
                description: "Progress indicators examples",
            },
            ExampleMeta {
                name: "CircularProgressIndicatorSample",
                description: "Progress indicators examples",
            },
            ExampleMeta {
                name: "CircularWavyProgressIndicatorSample",
                description: "Progress indicators examples",
            },
            ExampleMeta {
                name: "IndeterminateCircularProgressIndicatorSample",
                description: "Progress indicators examples",
            },
            ExampleMeta {
                name: "IndeterminateCircularWavyProgressIndicatorSample",
                description: "Progress indicators examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => example_content([
                example_row(
                    cx,
                    "30 %",
                    [div()
                        .w(px(360.))
                        .max_w_full()
                        .child(LinearProgress::new("linear-30").value(0.3))
                        .into_any_element()],
                ),
                example_row(
                    cx,
                    "70 %",
                    [div()
                        .w(px(360.))
                        .max_w_full()
                        .child(LinearProgress::new("linear-70").value(0.7))
                        .into_any_element()],
                ),
                example_row(
                    cx,
                    "Indeterminate",
                    [div()
                        .w(px(360.))
                        .max_w_full()
                        .child(LinearProgress::new("linear-indeterminate").indeterminate())
                        .into_any_element()],
                ),
            ]),
            1 => example_content([
                example_row(
                    cx,
                    "40 %",
                    [div()
                        .w(px(360.))
                        .max_w_full()
                        .child(LinearWavyProgressIndicator::new("wavy-40").value(0.4))
                        .into_any_element()],
                ),
                example_row(
                    cx,
                    "80 %",
                    [div()
                        .w(px(360.))
                        .max_w_full()
                        .child(LinearWavyProgressIndicator::new("wavy-80").value(0.8))
                        .into_any_element()],
                ),
                example_row(
                    cx,
                    "Indeterminate",
                    [div()
                        .w(px(360.))
                        .max_w_full()
                        .child(
                            LinearWavyProgressIndicator::new("wavy-indeterminate").indeterminate(),
                        )
                        .into_any_element()],
                ),
            ]),
            2 => example_content([example_row(
                cx,
                "Indeterminate",
                [div()
                    .w(px(360.))
                    .max_w_full()
                    .child(LinearProgress::new("linear-indeterminate").indeterminate())
                    .into_any_element()],
            )]),
            3 => example_content([example_row(
                cx,
                "Indeterminate",
                [div()
                    .w(px(360.))
                    .max_w_full()
                    .child(LinearWavyProgressIndicator::new("wavy-indeterminate").indeterminate())
                    .into_any_element()],
            )]),
            4 => example_content([
                example_row(
                    cx,
                    "Determinate",
                    [CircularProgress::new().size(px(48.)).into_any_element()],
                ),
                example_row(
                    cx,
                    "Indeterminate",
                    [CircularProgress::new().size(px(48.)).into_any_element()],
                ),
            ]),
            5 => example_content([
                example_row(
                    cx,
                    "Determinate",
                    [CircularWavyProgressIndicator::new("wavy-circular-40")
                        .value(0.4)
                        .into_any_element()],
                ),
                example_row(
                    cx,
                    "Indeterminate",
                    [
                        CircularWavyProgressIndicator::new("wavy-circular-indeterminate")
                            .into_any_element(),
                    ],
                ),
            ]),
            6 => example_content([example_row(
                cx,
                "Indeterminate",
                [CircularProgress::new().size(px(48.)).into_any_element()],
            )]),
            7 => example_content([example_row(
                cx,
                "Indeterminate",
                [
                    CircularWavyProgressIndicator::new("wavy-circular-indeterminate")
                        .into_any_element(),
                ],
            )]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
