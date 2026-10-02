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

const SHAPE_SAMPLES: usize = 128;
const OUTLINE_BUCKETS: usize = 512;
const ARC_SAMPLES: usize = 16;
const EDGE_SAMPLES: usize = 4;

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

    fn shape_radii(&self) -> &'static Vec<f32> {
        static CACHE: std::sync::OnceLock<Vec<Vec<f32>>> = std::sync::OnceLock::new();
        let cached =
            CACHE.get_or_init(|| Self::SEQUENCE.map(|shape| shape.compute_radii()).to_vec());
        let index = match self {
            MorphShape::SoftBurst => 0,
            MorphShape::Cookie9 => 1,
            MorphShape::Pentagon => 2,
            MorphShape::Pill => 3,
            MorphShape::Sunny => 4,
            MorphShape::Cookie4 => 5,
            MorphShape::SoftburstOval => 6,
        };
        &cached[index]
    }

    fn compute_radii(&self) -> Vec<f32> {
        let mut radii = match *self {
            MorphShape::SoftBurst => {
                rounded_polygon_radii(&star_vertices(10, 1.0, 0.65, 18.0_f32.to_radians()), 0.1)
            }
            MorphShape::Cookie9 => {
                rounded_polygon_radii(&star_vertices(9, 1.0, 0.8, -90.0_f32.to_radians()), 0.5)
            }
            MorphShape::Pentagon => rounded_polygon_radii(
                &regular_polygon_vertices(5, 1.0, -18.0_f32.to_radians()),
                0.3,
            ),
            MorphShape::Pill => (0..SHAPE_SAMPLES)
                .map(|i| {
                    rounded_rect_radius(
                        TAU * i as f32 / SHAPE_SAMPLES as f32,
                        0.625,
                        0.5,
                        0.5,
                        -45.0_f32.to_radians(),
                    )
                })
                .collect(),
            MorphShape::Sunny => rounded_polygon_radii(&star_vertices(8, 1.0, 0.8, 0.0), 0.15),
            MorphShape::Cookie4 => {
                rounded_polygon_radii(&star_vertices(4, 1.0, 0.5, -45.0_f32.to_radians()), 0.3)
            }
            MorphShape::SoftburstOval => (0..SHAPE_SAMPLES)
                .map(|i| {
                    ellipse_radius(
                        TAU * i as f32 / SHAPE_SAMPLES as f32,
                        1.0,
                        0.7,
                        -45.0_f32.to_radians(),
                    )
                })
                .collect(),
        };
        let mut max = 0.0_f32;
        for radius in &radii {
            max = max.max(*radius);
        }
        if max > 0.0 {
            for radius in &mut radii {
                *radius /= max;
            }
        }
        radii
    }
}

fn star_vertices(spikes: u32, outer: f32, inner: f32, rotation: f32) -> Vec<(f32, f32)> {
    let mut vertices = Vec::with_capacity(spikes as usize * 2);
    for k in 0..spikes {
        let outer_angle = rotation + k as f32 * TAU / spikes as f32;
        let inner_angle = outer_angle + std::f32::consts::PI / spikes as f32;
        vertices.push((outer * outer_angle.cos(), outer * outer_angle.sin()));
        vertices.push((inner * inner_angle.cos(), inner * inner_angle.sin()));
    }
    vertices
}

fn regular_polygon_vertices(sides: u32, radius: f32, rotation: f32) -> Vec<(f32, f32)> {
    (0..sides)
        .map(|k| {
            let angle = rotation + k as f32 * TAU / sides as f32;
            (radius * angle.cos(), radius * angle.sin())
        })
        .collect()
}

fn rounded_polygon_radii(vertices: &[(f32, f32)], corner_rounding: f32) -> Vec<f32> {
    let count = vertices.len();
    let mut buckets = vec![0.0_f32; OUTLINE_BUCKETS];
    let mut filled = vec![false; OUTLINE_BUCKETS];

    let emit = |buckets: &mut Vec<f32>, filled: &mut [bool], x: f32, y: f32| {
        let theta = y.atan2(x).rem_euclid(TAU);
        let bucket = ((theta / TAU) * OUTLINE_BUCKETS as f32) as usize % OUTLINE_BUCKETS;
        let radius = (x * x + y * y).sqrt();
        if radius > buckets[bucket] {
            buckets[bucket] = radius;
            filled[bucket] = true;
        }
    };

    for i in 0..count {
        let prev = vertices[(i + count - 1) % count];
        let cur = vertices[i];
        let next = vertices[(i + 1) % count];

        let in_x = prev.0 - cur.0;
        let in_y = prev.1 - cur.1;
        let out_x = next.0 - cur.0;
        let out_y = next.1 - cur.1;
        let in_len = (in_x * in_x + in_y * in_y).sqrt();
        let out_len = (out_x * out_x + out_y * out_y).sqrt();
        if in_len < 1.0e-6 || out_len < 1.0e-6 {
            continue;
        }
        let (in_x, in_y) = (in_x / in_len, in_y / in_len);
        let (out_x, out_y) = (out_x / out_len, out_y / out_len);

        let cos_corner = (in_x * out_x + in_y * out_y).clamp(-1.0, 1.0);
        let corner_angle = cos_corner.acos();
        let half = corner_angle * 0.5;
        if half < 1.0e-4 || (std::f32::consts::FRAC_PI_2 - half).abs() < 1.0e-4 {
            emit(&mut buckets, &mut filled, cur.0, cur.1);
            continue;
        }
        let mut tangent = corner_rounding / half.tan();
        tangent = tangent.min(in_len * 0.5).min(out_len * 0.5);
        if tangent < 1.0e-4 {
            emit(&mut buckets, &mut filled, cur.0, cur.1);
            continue;
        }
        let radius = tangent * half.tan();

        let start = (cur.0 + in_x * tangent, cur.1 + in_y * tangent);
        let end = (cur.0 + out_x * tangent, cur.1 + out_y * tangent);

        let bis_x = in_x + out_x;
        let bis_y = in_y + out_y;
        let bis_len = (bis_x * bis_x + bis_y * bis_y).sqrt();
        if bis_len < 1.0e-6 {
            continue;
        }
        let distance = radius / half.sin();
        let center = (
            cur.0 + bis_x / bis_len * distance,
            cur.1 + bis_y / bis_len * distance,
        );

        let start_angle = (start.1 - center.1).atan2(start.0 - center.0);
        let end_angle = (end.1 - center.1).atan2(end.0 - center.0);
        let corner_angle_polar = (cur.1 - center.1).atan2(cur.0 - center.0);
        let span_a = (end_angle - start_angle).rem_euclid(TAU);
        let span_b = (start_angle - end_angle).rem_euclid(TAU);
        let sweep = if ((start_angle + span_a * 0.5).rem_euclid(TAU) - corner_angle_polar)
            .rem_euclid(TAU)
            .abs()
            < TAU * 0.5
            && span_a <= span_b
        {
            span_a
        } else {
            TAU - span_b
        };
        let _ = sweep;
        let sweep = {
            let mid_a = start_angle + span_a * 0.5;
            let mid_b = end_angle + span_b * 0.5;
            let dist_a = (mid_a - corner_angle_polar).rem_euclid(TAU);
            let dist_b = (corner_angle_polar - mid_b).rem_euclid(TAU);
            if span_a.min(dist_a) <= span_b.min(dist_b.min(TAU)) || span_a <= span_b {
                span_a
            } else {
                TAU - span_b
            }
        };

        for s in 0..=ARC_SAMPLES {
            let t = s as f32 / ARC_SAMPLES as f32;
            let angle = start_angle + sweep * t;
            emit(
                &mut buckets,
                &mut filled,
                center.0 + radius * angle.cos(),
                center.1 + radius * angle.sin(),
            );
        }
        emit(&mut buckets, &mut filled, end.0, end.1);

        let next_index = (i + 1) % count;
        let next_start = if next_index == 0 {
            let (sx, sy) = corner_start(vertices, 0, corner_rounding);
            (sx, sy)
        } else {
            corner_start(vertices, next_index, corner_rounding)
        };
        for s in 1..EDGE_SAMPLES {
            let t = s as f32 / EDGE_SAMPLES as f32;
            emit(
                &mut buckets,
                &mut filled,
                end.0 + (next_start.0 - end.0) * t,
                end.1 + (next_start.1 - end.1) * t,
            );
        }
    }

    let filled_indices: Vec<usize> = (0..OUTLINE_BUCKETS).filter(|&i| filled[i]).collect();
    if filled_indices.is_empty() {
        return vec![1.0; SHAPE_SAMPLES];
    }
    for k in 0..filled_indices.len() {
        let current = filled_indices[k];
        let next = filled_indices[(k + 1) % filled_indices.len()];
        let span = if next > current {
            next - current
        } else {
            OUTLINE_BUCKETS - current + next
        };
        for step in 1..span {
            let bucket = (current + step) % OUTLINE_BUCKETS;
            let t = step as f32 / span as f32;
            buckets[bucket] = buckets[current] + (buckets[next] - buckets[current]) * t;
        }
    }

    (0..SHAPE_SAMPLES)
        .map(|i| {
            let theta = TAU * i as f32 / SHAPE_SAMPLES as f32;
            let bucket = ((theta / TAU) * OUTLINE_BUCKETS as f32) as usize % OUTLINE_BUCKETS;
            buckets[bucket]
        })
        .collect()
}

fn corner_start(vertices: &[(f32, f32)], index: usize, corner_rounding: f32) -> (f32, f32) {
    let count = vertices.len();
    let prev = vertices[(index + count - 1) % count];
    let cur = vertices[index];
    let in_x = prev.0 - cur.0;
    let in_y = prev.1 - cur.1;
    let in_len = (in_x * in_x + in_y * in_y).sqrt();
    if in_len < 1.0e-6 {
        return cur;
    }
    let (in_x, in_y) = (in_x / in_len, in_y / in_len);
    let out_x = vertices[(index + 1) % count].0 - cur.0;
    let out_y = vertices[(index + 1) % count].1 - cur.1;
    let out_len = (out_x * out_x + out_y * out_y).sqrt();
    if out_len < 1.0e-6 {
        return cur;
    }
    let (out_x, out_y) = (out_x / out_len, out_y / out_len);
    let cos_corner = (in_x * out_x + in_y * out_y).clamp(-1.0, 1.0);
    let half = cos_corner.acos() * 0.5;
    let mut tangent = corner_rounding / half.tan();
    tangent = tangent.min(in_len * 0.5);
    (cur.0 + in_x * tangent, cur.1 + in_y * tangent)
}

fn sample_radii(shape: MorphShape) -> Vec<f32> {
    shape.shape_radii().clone()
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

    let count = outline.len();
    let mut builder = PathBuilder::fill();
    builder.move_to(outline[0]);
    for i in 0..count {
        let p0 = outline[(i + count - 1) % count];
        let p1 = outline[i];
        let p2 = outline[(i + 1) % count];
        let p3 = outline[(i + 2) % count];
        let control_a = point(p1.x + (p2.x - p0.x) / 6.0, p1.y + (p2.y - p0.y) / 6.0);
        let control_b = point(p2.x - (p3.x - p1.x) / 6.0, p2.y - (p3.y - p1.y) / 6.0);
        builder.cubic_bezier_to(p2, control_a, control_b);
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
    fn star_vertices_alternate_radii() {
        let vertices = star_vertices(4, 1.0, 0.5, 0.0);
        assert_eq!(vertices.len(), 8);
        let first = (vertices[0].0 * vertices[0].0 + vertices[0].1 * vertices[0].1).sqrt();
        let second = (vertices[1].0 * vertices[1].0 + vertices[1].1 * vertices[1].1).sqrt();
        assert!((first - 1.0).abs() < 1.0e-4);
        assert!((second - 0.5).abs() < 1.0e-4);
    }

    #[test]
    fn rounded_cookie4_stays_within_unit_circle() {
        let radii = MorphShape::Cookie4.compute_radii();
        assert!(radii.iter().all(|r| *r <= 1.0 + 1.0e-3));
        assert!(radii.iter().all(|r| r.is_finite() && *r > 0.0));
    }
}
