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
// 参考 https://github.com/Glavo/m3fx/blob/main/demo/src/main/java/org/glavo/m3fx/demo/LoadingIndicatorDemoPage.java

use gpui::{App, Entity, IntoElement, Render, Window, div, prelude::*, px};
use material3_gpui::prelude::*;

use super::{page, showcase_group, specimen};

pub struct LoadingIndicatorsPage;

impl LoadingIndicatorsPage {
    pub fn new(_cx: &mut App) -> Entity<Self> {
        _cx.new(|_| Self)
    }
}

impl Render for LoadingIndicatorsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        page(
            cx,
            "Loading indicators",
            "Expressive morphing shapes; the contained variant wraps the shape in a tonal circle.",
            [
                showcase_group(
                    cx,
                    "Default",
                    [
                        div()
                            .size(px(112.))
                            .flex_none()
                            .child(LoadingIndicator::new("loading-xl").size(px(112.)))
                            .into_any_element(),
                        specimen(
                            cx,
                            "48 dp",
                            LoadingIndicator::new("loading-48").size(px(48.)),
                        ),
                        specimen(
                            cx,
                            "32 dp",
                            LoadingIndicator::new("loading-32").size(px(32.)),
                        ),
                    ],
                )
                .into_any_element(),
                showcase_group(
                    cx,
                    "Contained",
                    [
                        div()
                            .size(px(112.))
                            .flex_none()
                            .child(ContainedLoadingIndicator::new("contained-xl").size(px(112.)))
                            .into_any_element(),
                        specimen(
                            cx,
                            "48 dp",
                            ContainedLoadingIndicator::new("contained-48").size(px(48.)),
                        ),
                    ],
                )
                .into_any_element(),
            ],
        )
    }
}
