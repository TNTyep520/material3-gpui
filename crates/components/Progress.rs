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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/ProgressIndicator.kt

use gpui::{
    Animation, AnimationExt, App, ElementId, IntoElement, RenderOnce, Transformation, Window, div,
    ease_in_out, percentage, prelude::*, relative, svg,
};
use std::time::Duration;

use crate::theme::ActiveTheme;

#[derive(IntoElement)]
pub struct LinearProgress {
    id: ElementId,

    value: Option<f32>,
}

pub type LinearProgressIndicator = LinearProgress;

impl LinearProgress {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: None,
        }
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = Some(value.clamp(0., 1.));
        self
    }

    pub fn indeterminate(mut self) -> Self {
        self.value = None;
        self
    }
}

impl RenderOnce for LinearProgress {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let style = LinearProgressStyle::resolve(cx.theme().token_set());
        let active = style.active_color;
        let track = style.track_color;

        let container = div()
            .id(self.id)
            .w_full()
            .h(style.height)
            .rounded(style.corner_radius)
            .bg(track)
            .overflow_hidden();

        match self.value {
            Some(value) => {
                container.child(div().h_full().w(relative(value)).rounded_full().bg(active))
            }
            None => container.child(
                div().relative().size_full().child(
                    div()
                        .absolute()
                        .top_0()
                        .h_full()
                        .w(relative(0.4))
                        .rounded_full()
                        .bg(active)
                        .with_animation(
                            "md3-linear-indeterminate",
                            Animation::new(Duration::from_millis(1200))
                                .repeat()
                                .with_easing(ease_in_out),
                            |el, delta| el.left(relative(-0.4 + delta * 1.4)),
                        ),
                ),
            ),
        }
    }
}

#[derive(IntoElement)]
pub struct CircularProgress {
    size: Option<gpui::Pixels>,
}

pub type CircularProgressIndicator = CircularProgress;

impl CircularProgress {
    pub fn new() -> Self {
        Self { size: None }
    }

    pub fn size(mut self, size: gpui::Pixels) -> Self {
        self.size = Some(size);
        self
    }
}

impl Default for CircularProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for CircularProgress {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let style = CircularProgressStyle::resolve(cx.theme().token_set());
        svg()
            .path(crate::assets::PROGRESS_ARC_SVG_PATH)
            .size(self.size.unwrap_or(style.size))
            .text_color(style.color)
            .with_animation(
                "md3-circular-indeterminate",
                Animation::new(Duration::from_millis(1000)).repeat(),
                |el, delta| el.with_transformation(Transformation::rotate(percentage(delta))),
            )
    }
}

pub use appearance::{CircularProgressStyle, LinearProgressStyle};

mod appearance {
    use crate::theme::TokenSet;
    use gpui::{Hsla, Pixels};

    #[derive(Clone, Copy, Debug)]
    pub struct LinearProgressStyle {
        pub active_color: Hsla,

        pub track_color: Hsla,

        pub height: Pixels,

        pub corner_radius: Pixels,
    }
    impl LinearProgressStyle {
        pub fn resolve(tokens: &TokenSet) -> Self {
            use crate::tokens::{LinearProgressIndicatorTokens, ProgressIndicatorTokens};
            Self {
                active_color: ProgressIndicatorTokens::ACTIVE_INDICATOR_COLOR.resolve(tokens),
                track_color: ProgressIndicatorTokens::TRACK_COLOR.resolve(tokens),
                height: LinearProgressIndicatorTokens::HEIGHT.pixels(),
                corner_radius: tokens.shapes.full,
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct CircularProgressStyle {
        pub color: Hsla,

        pub size: Pixels,
    }
    impl CircularProgressStyle {
        pub fn resolve(tokens: &TokenSet) -> Self {
            use crate::tokens::{CircularProgressIndicatorTokens, ProgressIndicatorTokens};
            Self {
                color: ProgressIndicatorTokens::ACTIVE_INDICATOR_COLOR.resolve(tokens),
                size: CircularProgressIndicatorTokens::SIZE.pixels(),
            }
        }
    }
}
