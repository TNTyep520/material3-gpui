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

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use gpui::{
    Animation, AnimationExt, App, ElementId, IntoElement, PathBuilder, Pixels, RenderOnce,
    Transformation, Window, canvas, div, ease_in_out, percentage, point, prelude::*, px, relative,
    svg,
};

use crate::motion::easing;
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

const TAU: f32 = std::f32::consts::TAU;

const LINEAR_WAVE_PHASE_MILLIS: u64 = 1000;
const LINEAR_INDETERMINATE_MILLIS: u64 = 1750;
const CIRCULAR_INDETERMINATE_MILLIS: u64 = 6000;

#[derive(IntoElement)]
pub struct LinearWavyProgressIndicator {
    id: ElementId,
    value: Option<f32>,
}

pub type WavyProgressIndicator = LinearWavyProgressIndicator;

impl LinearWavyProgressIndicator {
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

impl RenderOnce for LinearWavyProgressIndicator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        use crate::tokens::{LinearProgressIndicatorTokens as Tokens, ProgressIndicatorTokens};
        let tokens = cx.theme().token_set();
        let active = ProgressIndicatorTokens::ACTIVE_INDICATOR_COLOR.resolve(tokens);
        let track_color = ProgressIndicatorTokens::TRACK_COLOR.resolve(tokens);
        let thickness = Tokens::ACTIVE_THICKNESS.pixels();
        let amplitude = Tokens::ACTIVE_WAVE_AMPLITUDE.pixels();
        let wave_height = Tokens::WAVE_HEIGHT.pixels();
        let wavelength = match self.value {
            Some(_) => Tokens::ACTIVE_WAVE_WAVELENGTH.pixels(),
            None => Tokens::INDETERMINATE_ACTIVE_WAVE_WAVELENGTH.pixels(),
        };
        let value = self.value;
        let phase_cell = Rc::new(Cell::new(0.0_f32));
        let paint_phase = phase_cell.clone();

        let wave = canvas(
            move |_, _, _| paint_phase,
            move |bounds, phase, window, _| {
                let phase = phase.get();
                let width = f32::from(bounds.size.width);
                let middle = bounds.size.height * 0.5;

                let paint_wave = |builder: &mut PathBuilder, start: f32, end: f32, amp: f32| {
                    let mut top = Vec::new();
                    let mut x = start;
                    while x <= end {
                        let offset = amp * (TAU * (x / f32::from(wavelength) - phase)).sin();
                        top.push((x, middle + px(offset) - thickness * 0.5));
                        x += 3.0;
                    }
                    if top.len() < 2 {
                        return;
                    }
                    for (i, (x, y)) in top.iter().enumerate() {
                        let point = point(px(*x), *y);
                        if i == 0 {
                            builder.move_to(point);
                        } else {
                            builder.line_to(point);
                        }
                    }
                    for (x, y) in top.iter().rev() {
                        builder.line_to(point(px(*x), *y + thickness));
                    }
                    builder.close();
                };

                match value {
                    Some(v) => {
                        let end = width * v;
                        let gap = f32::from(px(4.));
                        let stop = f32::from(Tokens::STOP_SIZE.pixels());
                        let amp = f32::from(amplitude);
                        let mut builder = PathBuilder::fill();
                        paint_wave(&mut builder, 0.0, end, amp);
                        if end + gap < width {
                            builder.move_to(point(px(end + gap), middle - thickness * 0.5));
                            builder.line_to(point(px(width), middle - thickness * 0.5));
                            builder.line_to(point(px(width), middle + thickness * 0.5));
                            builder.line_to(point(px(end + gap), middle + thickness * 0.5));
                            builder.close();
                        }
                        if end + gap + stop <= width {
                            builder.move_to(point(px(width - stop), middle - thickness * 0.5));
                            builder.line_to(point(px(width), middle - thickness * 0.5));
                            builder.line_to(point(px(width), middle + thickness * 0.5));
                            builder.line_to(point(px(width - stop), middle + thickness * 0.5));
                            builder.close();
                        }
                        if let Ok(path) = builder.build() {
                            window.paint_path(path, active);
                        }
                    }
                    None => {
                        let t = phase;
                        let accelerate = |x: f32| {
                            easing::EMPHASIZED_ACCELERATE.sample(f64::from(x.clamp(0.0, 1.0)))
                                as f32
                        };
                        let head = accelerate(t / 1.75);
                        let tail = accelerate((t - 0.25) / 1.75);
                        let (start, end) = (width * tail.min(head), width * head);
                        if end - start < 2.0 {
                            return;
                        }
                        let amp = f32::from(amplitude);
                        let mut builder = PathBuilder::fill();
                        paint_wave(&mut builder, start, end, amp);
                        if let Ok(path) = builder.build() {
                            window.paint_path(path, active);
                        }
                    }
                }
            },
        );

        let animation_phase = phase_cell;
        div()
            .id(self.id.clone())
            .relative()
            .w_full()
            .flex_none()
            .h(wave_height)
            .when(value.is_none(), |el| {
                el.child(
                    div()
                        .absolute()
                        .top((wave_height - thickness) * 0.5)
                        .left_0()
                        .right_0()
                        .h(thickness)
                        .rounded_full()
                        .bg(track_color),
                )
            })
            .child(wave.absolute().size_full())
            .with_animation(
                self.id,
                Animation::new(Duration::from_millis(match value {
                    Some(_) => LINEAR_WAVE_PHASE_MILLIS,
                    None => LINEAR_INDETERMINATE_MILLIS,
                }))
                .repeat(),
                move |el, delta| {
                    animation_phase.set(delta);
                    el
                },
            )
    }
}

#[derive(IntoElement)]
pub struct CircularWavyProgressIndicator {
    id: ElementId,
    value: Option<f32>,
    size: Pixels,
}

pub type WavyCircularProgressIndicator = CircularWavyProgressIndicator;

impl CircularWavyProgressIndicator {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: None,
            size: crate::tokens::CircularProgressIndicatorTokens::WAVE_SIZE.pixels(),
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

    pub fn size(mut self, size: Pixels) -> Self {
        self.size = size;
        self
    }
}

impl RenderOnce for CircularWavyProgressIndicator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        use crate::tokens::{CircularProgressIndicatorTokens as Tokens, ProgressIndicatorTokens};
        let tokens = cx.theme().token_set();
        let active = ProgressIndicatorTokens::ACTIVE_INDICATOR_COLOR.resolve(tokens);
        let track_color = ProgressIndicatorTokens::TRACK_COLOR.resolve(tokens);
        let thickness = Tokens::ACTIVE_THICKNESS.pixels();
        let amplitude = Tokens::ACTIVE_WAVE_AMPLITUDE.pixels();
        let wavelength = Tokens::ACTIVE_WAVE_WAVELENGTH.pixels();
        let value = self.value;
        let box_size = self.size;
        let phase_cell = Rc::new(Cell::new(0.0_f32));
        let paint_phase = phase_cell.clone();

        let wave = canvas(
            move |_, _, _| paint_phase,
            move |bounds, phase, window, _| {
                let phase = phase.get();
                let diameter = f32::from(bounds.size.width.min(bounds.size.height));
                let center = bounds.center();
                let base_radius = (diameter - f32::from(thickness)) * 0.5 - f32::from(amplitude);
                if base_radius <= 0.0 {
                    return;
                }
                let wave_count = ((TAU * base_radius / f32::from(wavelength)) as u32).clamp(5, 18);
                let (rotation, sweep) = match value {
                    Some(_) => (0.0, 1.0),
                    None => {
                        let triangle = 1.0 - (2.0 * phase - 1.0).abs();
                        (phase * 3.0 * TAU, 0.10 + 0.77 * triangle)
                    }
                };
                let sample_count = 96;
                let arc_points = ((sweep * sample_count as f32) as usize).max(2);
                let mut outer = Vec::with_capacity(arc_points);
                let mut inner = Vec::with_capacity(arc_points);
                for i in 0..arc_points {
                    let fraction = i as f32 / (arc_points - 1) as f32;
                    let edge_distance = fraction.min(1.0 - fraction) * sweep * wave_count as f32;
                    let taper = (edge_distance / 0.5).min(1.0);
                    let wave_theta = TAU * (fraction * wave_count as f32 - phase);
                    let offset = f32::from(amplitude) * taper * wave_theta.sin();
                    let angle = -std::f32::consts::FRAC_PI_2 + rotation + fraction * sweep * TAU;
                    let (sin, cos) = angle.sin_cos();
                    let radial = base_radius + offset;
                    let half = f32::from(thickness) * 0.5;
                    outer.push(point(
                        center.x + px((radial + half) * cos),
                        center.y + px((radial + half) * sin),
                    ));
                    inner.push(point(
                        center.x + px((radial - half) * cos),
                        center.y + px((radial - half) * sin),
                    ));
                }
                let mut builder = PathBuilder::fill();
                for (i, point) in outer.iter().enumerate() {
                    if i == 0 {
                        builder.move_to(*point);
                    } else {
                        builder.line_to(*point);
                    }
                }
                for point in inner.iter().rev() {
                    builder.line_to(*point);
                }
                builder.close();
                if let Ok(path) = builder.build() {
                    window.paint_path(path, active);
                }
            },
        );

        let animation_phase = phase_cell;
        div()
            .id(self.id.clone())
            .size(box_size)
            .flex_none()
            .relative()
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .rounded_full()
                    .border(thickness)
                    .border_color(track_color),
            )
            .child(wave.absolute().size_full())
            .with_animation(
                self.id,
                Animation::new(Duration::from_millis(match value {
                    Some(_) => LINEAR_WAVE_PHASE_MILLIS,
                    None => CIRCULAR_INDETERMINATE_MILLIS,
                }))
                .repeat(),
                move |el, delta| {
                    animation_phase.set(delta);
                    el
                },
            )
    }
}
