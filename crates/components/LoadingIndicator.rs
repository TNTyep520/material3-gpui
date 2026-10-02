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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/LoadingIndicator.kt
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/ProgressIndicator.kt

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use gpui::{
    Animation, AnimationExt, App, ElementId, Hsla, IntoElement, PathBuilder, Pixels, Point,
    RenderOnce, Styled, Window, canvas, point, px,
};

use crate::motion::easing;
use crate::theme::ActiveTheme;

const TAU: f32 = std::f32::consts::TAU;

const LOADING_SHAPE_COUNT: usize = 7;
const LOADING_SEGMENT_MILLIS: u64 = 650;
const LOADING_MORPH_ACTIVE_FRACTION: f32 = 0.72;
const LOADING_BREATHING_AMPLITUDE: f32 = 0.12;

const SHAPE_SAMPLES: usize = 72;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoadingIndicatorVariant {
    Default,
    Contained,
}

#[derive(IntoElement)]
pub struct LoadingIndicator {
    id: ElementId,
    size: Pixels,
    variant: LoadingIndicatorVariant,
}

#[derive(IntoElement)]
pub struct ContainedLoadingIndicator(LoadingIndicator);

impl ContainedLoadingIndicator {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self(LoadingIndicator::new(id).variant(LoadingIndicatorVariant::Contained))
    }

    pub fn size(mut self, size: Pixels) -> Self {
        self.0 = self.0.size(size);
        self
    }
}

impl RenderOnce for ContainedLoadingIndicator {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.0.render(window, cx)
    }
}

impl LoadingIndicator {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            size: px(48.),
            variant: LoadingIndicatorVariant::Default,
        }
    }

    pub fn size(mut self, size: Pixels) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: LoadingIndicatorVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn contained(mut self) -> Self {
        self.variant = LoadingIndicatorVariant::Contained;
        self
    }
}

impl RenderOnce for LoadingIndicator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        use crate::tokens::LoadingIndicatorTokens as Tokens;
        let tokens = cx.theme().token_set();
        let fill = match self.variant {
            LoadingIndicatorVariant::Default => Tokens::ACTIVE_INDICATOR_COLOR.resolve(tokens),
            LoadingIndicatorVariant::Contained => Tokens::CONTAINED_ACTIVE_COLOR.resolve(tokens),
        };
        let container_bg = match self.variant {
            LoadingIndicatorVariant::Default => gpui::transparent_black(),
            LoadingIndicatorVariant::Contained => Tokens::CONTAINED_CONTAINER_COLOR.resolve(tokens),
        };
        let active_ratio = Tokens::ACTIVE_SIZE.pixels() / Tokens::CONTAINER_WIDTH.pixels();

        let progress = Rc::new(Cell::new(0.0_f32));
        let paint_progress = progress.clone();
        let shape = canvas(
            move |_, _, _| paint_progress,
            move |bounds, progress, window, _| {
                let progress = progress.get();
                let radius = f32::from(bounds.size.width.min(bounds.size.height)) * 0.5;
                let center = bounds.center();
                draw_morphing_shape(window, center, radius * active_ratio, progress, fill);
            },
        )
        .size(self.size)
        .rounded_full()
        .bg(container_bg);

        let animation_progress = progress;
        shape.with_animation(
            self.id,
            Animation::new(Duration::from_millis(
                LOADING_SEGMENT_MILLIS * LOADING_SHAPE_COUNT as u64,
            ))
            .repeat(),
            move |el, delta| {
                animation_progress.set(delta);
                el
            },
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MorphShape {
    SoftBurst,
    Cookie9,
    Pentagon,
    Pill,
    Sunny,
    Cookie4,
    SoftburstOval,
}

impl MorphShape {
    const SEQUENCE: [MorphShape; LOADING_SHAPE_COUNT] = [
        MorphShape::SoftBurst,
        MorphShape::Cookie9,
        MorphShape::Pentagon,
        MorphShape::Pill,
        MorphShape::Sunny,
        MorphShape::Cookie4,
        MorphShape::SoftburstOval,
    ];

    fn radius_at(&self, theta: f32) -> f32 {
        match *self {
            MorphShape::SoftBurst => star_radius(theta, 10, 1.0, 0.65, 18.0_f32.to_radians()),
            MorphShape::Cookie9 => star_radius(theta, 9, 1.0, 0.8, -90.0_f32.to_radians()),
            MorphShape::Pentagon => star_radius(theta, 5, 1.0, 1.0, -18.0_f32.to_radians()),
            MorphShape::Pill => rounded_rect_radius(theta, 0.625, 0.5, 0.5, -45.0_f32.to_radians()),
            MorphShape::Sunny => star_radius(theta, 8, 1.0, 0.8, 0.0),
            MorphShape::Cookie4 => star_radius(theta, 4, 1.0, 0.5, -45.0_f32.to_radians()),
            MorphShape::SoftburstOval => ellipse_radius(theta, 1.0, 0.7, -45.0_f32.to_radians()),
        }
    }
}

fn sample_radii(shape: MorphShape) -> Vec<f32> {
    let mut radii = Vec::with_capacity(SHAPE_SAMPLES);
    let mut max = 0.0_f32;
    for i in 0..SHAPE_SAMPLES {
        let theta = TAU * i as f32 / SHAPE_SAMPLES as f32;
        let radius = shape.radius_at(theta);
        max = max.max(radius);
        radii.push(radius);
    }
    if max > 0.0 {
        for radius in &mut radii {
            *radius /= max;
        }
    }
    radii
}

fn star_radius(theta: f32, spikes: u32, outer: f32, inner: f32, rotation: f32) -> f32 {
    let step = std::f32::consts::PI / spikes as f32;
    let local = (theta - rotation).rem_euclid(TAU);
    let index = (local / step).floor() as u32;
    let angle_a = rotation + index as f32 * step;
    let angle_b = angle_a + step;
    let radius_a = if index.is_multiple_of(2) {
        outer
    } else {
        inner
    };
    let radius_b = if index.is_multiple_of(2) {
        inner
    } else {
        outer
    };
    chord_radius(theta, angle_a, radius_a, angle_b, radius_b)
}

fn chord_radius(theta: f32, angle_a: f32, radius_a: f32, angle_b: f32, radius_b: f32) -> f32 {
    let (sin_a, cos_a) = angle_a.sin_cos();
    let (sin_b, cos_b) = angle_b.sin_cos();
    let x_a = radius_a * cos_a;
    let y_a = radius_a * sin_a;
    let x_b = radius_b * cos_b;
    let y_b = radius_b * sin_b;
    let denominator = (y_b - y_a) * theta.cos() - (x_b - x_a) * theta.sin();
    if denominator.abs() < 1.0e-6 {
        return radius_a.max(radius_b);
    }
    let cross = x_a * y_b - x_b * y_a;
    let radius = cross / denominator;
    if radius > 0.0 {
        radius
    } else {
        radius_a.max(radius_b)
    }
}

fn rounded_rect_radius(
    theta: f32,
    half_width: f32,
    half_height: f32,
    corner: f32,
    rotation: f32,
) -> f32 {
    let angle = theta - rotation;
    let (sin, cos) = angle.sin_cos();
    let mut radius = f32::INFINITY;
    if cos > 1.0e-6 {
        radius = radius.min(half_width / cos);
    }
    if cos < -1.0e-6 {
        radius = radius.min(-half_width / cos);
    }
    if sin > 1.0e-6 {
        radius = radius.min(half_height / sin);
    }
    if sin < -1.0e-6 {
        radius = radius.min(-half_height / sin);
    }
    let center_x = (half_width - corner) * cos.signum();
    let center_y = (half_height - corner) * sin.signum();
    let along = center_x * cos + center_y * sin;
    let perpendicular = (center_x * sin - center_y * cos).abs();
    let discriminant = corner * corner - perpendicular * perpendicular;
    if discriminant > 0.0 && along >= 0.0 {
        radius = radius.min(along + discriminant.sqrt());
    }
    if radius.is_finite() {
        radius
    } else {
        half_width.max(half_height)
    }
}

fn ellipse_radius(theta: f32, semi_major: f32, semi_minor: f32, rotation: f32) -> f32 {
    let angle = theta - rotation;
    let (sin, cos) = angle.sin_cos();
    let denominator = ((semi_minor * cos).powi(2) + (semi_major * sin).powi(2)).sqrt();
    if denominator < 1.0e-6 {
        return semi_major;
    }
    semi_major * semi_minor / denominator
}

fn chaikin_smooth(points: &[Point<Pixels>], iterations: usize) -> Vec<Point<Pixels>> {
    let mut current = points.to_vec();
    for _ in 0..iterations {
        let source = current.clone();
        current.clear();
        let count = source.len();
        for i in 0..count {
            let a = source[i];
            let b = source[(i + 1) % count];
            current.push(point(a.x + (b.x - a.x) * 0.75, a.y + (b.y - a.y) * 0.75));
            current.push(point(a.x + (b.x - a.x) * 0.25, a.y + (b.y - a.y) * 0.25));
        }
    }
    current
}

fn draw_morphing_shape(
    window: &mut Window,
    center: Point<Pixels>,
    radius: f32,
    progress: f32,
    color: Hsla,
) {
    let cycle = progress * LOADING_SHAPE_COUNT as f32;
    let index = (cycle.floor() as usize).min(LOADING_SHAPE_COUNT - 1);
    let segment = cycle.fract();
    let morph_input = (segment / LOADING_MORPH_ACTIVE_FRACTION).min(1.0);
    let morph = easing::EXPRESSIVE_DEFAULT_SPATIAL.sample(f64::from(morph_input)) as f32;
    let breathing =
        1.0 + LOADING_BREATHING_AMPLITUDE * (std::f32::consts::PI * segment).sin().powi(2);
    let from = sample_radii(MorphShape::SEQUENCE[index]);
    let to = sample_radii(MorphShape::SEQUENCE[(index + 1) % LOADING_SHAPE_COUNT]);
    let rotation = progress * TAU;

    let scale = radius * breathing;
    let mut outline = Vec::with_capacity(SHAPE_SAMPLES);
    for i in 0..SHAPE_SAMPLES {
        let theta = TAU * i as f32 / SHAPE_SAMPLES as f32;
        let interpolated = from[i] + (to[i] - from[i]) * morph;
        let angle = theta + rotation;
        let (sin, cos) = angle.sin_cos();
        outline.push(point(
            center.x + px(interpolated * scale * cos),
            center.y + px(interpolated * scale * sin),
        ));
    }

    let mut builder = PathBuilder::fill();
    for (i, point) in chaikin_smooth(&outline, 2).into_iter().enumerate() {
        if i == 0 {
            builder.move_to(point);
        } else {
            builder.line_to(point);
        }
    }
    builder.close();
    if let Ok(path) = builder.build() {
        window.paint_path(path, color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn morph_sequence_samples_are_normalized() {
        for shape in MorphShape::SEQUENCE {
            let radii = sample_radii(shape);
            assert_eq!(radii.len(), SHAPE_SAMPLES);
            assert!(radii.iter().all(|r| r.is_finite() && *r > 0.0));
            let max = radii.iter().cloned().fold(0.0, f32::max);
            assert!((max - 1.0).abs() < 1.0e-3);
        }
    }

    #[test]
    fn star_radius_hits_vertices() {
        let outer = star_radius(0.0, 4, 1.0, 0.5, 0.0);
        assert!((outer - 1.0).abs() < 1.0e-4);
        let inner = star_radius(std::f32::consts::PI / 4.0, 4, 1.0, 0.5, 0.0);
        assert!((inner - 0.5).abs() < 1.0e-4);
    }
}
