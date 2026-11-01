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

pub struct ScrollbarPage;

impl ScrollbarPage {
    pub fn new(_cx: &mut App) -> Entity<Self> {
        _cx.new(|_| Self)
    }
}

/// 把 Scrollbar 放进固定尺寸的相对定位容器,便于观察滑块位置。
fn scrollbar_container(id: &'static str, position: f32) -> AnyElement {
    div()
        .w(px(280.))
        .h(px(72.))
        .relative()
        .child(Scrollbar::new(id).position(position))
        .into_any_element()
}

impl ComponentPage for ScrollbarPage {
    fn description(&self) -> &'static str {
        "Scrollbars indicate the current scroll position within scrollable content."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "NonInteractiveScrollbarWithLazyColumnSample",
                description: "Scrollbar examples",
            },
            ExampleMeta {
                name: "NonInteractiveScrollbarWithVerticalScrollSample",
                description: "Scrollbar examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => example_content([example_row(
                cx,
                "Static",
                [scrollbar_container("scrollbar-static", 0.35)],
            )]),
            1 => example_content([example_row(
                cx,
                "Positions",
                [
                    scrollbar_container("scrollbar-10", 0.1),
                    scrollbar_container("scrollbar-50", 0.5),
                    scrollbar_container("scrollbar-90", 0.9),
                ],
            )]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
