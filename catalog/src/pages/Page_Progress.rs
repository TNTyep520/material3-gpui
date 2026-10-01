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

use gpui::{App, Entity, IntoElement, Render, Window, prelude::*, px};
use material3_gpui::prelude::*;

use super::{full_width_group, page, showcase_group};

pub struct ProgressPage;

impl ProgressPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self)
    }
}

impl Render for ProgressPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        page(
            cx,
            "Progress",
            "Linear and circular indicators, including the expressive wavy variants.",
            [
                full_width_group(
                    cx,
                    "Linear",
                    [
                        LinearProgress::new("linear-30").value(0.3),
                        LinearProgress::new("linear-70").value(0.7),
                        LinearProgress::new("linear-indeterminate").indeterminate(),
                    ],
                )
                .into_any_element(),
                showcase_group(cx, "Circular", [CircularProgress::new().size(px(48.))])
                    .into_any_element(),
                full_width_group(
                    cx,
                    "Wavy linear",
                    [
                        LinearWavyProgressIndicator::new("wavy-40").value(0.4),
                        LinearWavyProgressIndicator::new("wavy-80").value(0.8),
                        LinearWavyProgressIndicator::new("wavy-indeterminate").indeterminate(),
                    ],
                )
                .into_any_element(),
                showcase_group(
                    cx,
                    "Wavy circular",
                    [
                        CircularWavyProgressIndicator::new("wavy-circular-40").value(0.4),
                        CircularWavyProgressIndicator::new("wavy-circular-indeterminate"),
                    ],
                )
                .into_any_element(),
            ],
        )
    }
}
