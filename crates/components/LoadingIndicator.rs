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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/MaterialShapes.kt
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/skins/M3LoadingIndicatorSkin.java

use std::cell::Cell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use gpui::{
    AnyElement, App, Element, ElementId, GlobalElementId, Hsla, InspectorElementId, IntoElement,
    LayoutId, PathBuilder, Pixels, Point, RenderOnce, Window, canvas, div, point, prelude::*, px,
};

use crate::motion::MotionRole;
use crate::shape::{CornerRounding, Cubic, Morph, RoundedPolygon};
use crate::theme::ActiveTheme;

const TAU: f32 = std::f32::consts::TAU;

const MORPH_INTERVAL_MILLIS: u64 = 650;
const MORPH_ACTIVE_FRACTION: f32 = 0.72;
const GLOBAL_ROTATION_MILLIS: u64 = 4666;
const QUARTER_TURN: f32 = 0.25;
const MORPH_SCALE_AMPLITUDE: f32 = 0.12;
const INDICATOR_TO_CONTAINER: f32 = 38.0 / 48.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoadingIndicatorVariant {
    Default,
    Contained,
}

#[derive(IntoElement)]
pub struct LoadingIndicator {
    id: ElementId,
    size: Pixels,
    indicator_size: Pixels,
    variant: LoadingIndicatorVariant,
    progress: Option<f32>,
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

    pub fn indicator_size(mut self, size: Pixels) -> Self {
        self.0 = self.0.indicator_size(size);
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
            indicator_size: px(48. * INDICATOR_TO_CONTAINER),
            variant: LoadingIndicatorVariant::Default,
            progress: None,
        }
    }

    pub fn size(mut self, size: Pixels) -> Self {
        self.size = size;
        self.indicator_size = size * INDICATOR_TO_CONTAINER;
        self
    }

    pub fn indicator_size(mut self, size: Pixels) -> Self {
        self.indicator_size = size;
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

    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = Some(progress.clamp(0.0, 1.0));
        self
    }
}

struct ShapeSet {
    morphs: Vec<Morph>,
    centers: Vec<((f32, f32), (f32, f32))>,
    scale_factor: f32,
}

fn sequence_scale(polygons: &[RoundedPolygon]) -> f32 {
    let mut scale = 1.0_f32;
    for polygon in polygons {
        let (bx0, by0, bx1, by1) = polygon.calculate_bounds(false);
        let (mx0, my0, mx1, my1) = polygon.calculate_max_bounds();
        let scale_x = (bx1 - bx0) / (mx1 - mx0);
        let scale_y = (by1 - by0) / (my1 - my0);
        scale = scale.min(scale_x.max(scale_y));
    }
    scale
}

fn rotate_about(x: f32, y: f32, degrees: f32, center: (f32, f32)) -> (f32, f32) {
    let radians = degrees.to_radians();
    let (sin, cos) = radians.sin_cos();
    let dx = x - center.0;
    let dy = y - center.1;
    (
        center.0 + dx * cos - dy * sin,
        center.1 + dx * sin + dy * cos,
    )
}

fn repeat_points(
    points: &[((f32, f32), CornerRounding)],
    reps: usize,
    center: (f32, f32),
    mirroring: bool,
) -> Vec<((f32, f32), CornerRounding)> {
    let np = points.len();
    if mirroring {
        let angles: Vec<f32> = points
            .iter()
            .map(|(point, _)| (point.1 - center.1).atan2(point.0 - center.0).to_degrees())
            .collect();
        let distances: Vec<f32> = points
            .iter()
            .map(|(point, _)| (point.0 - center.0).hypot(point.1 - center.1))
            .collect();
        let actual_reps = reps * 2;
        let section_angle = 360.0 / actual_reps as f32;
        let mut out = Vec::with_capacity(np * actual_reps);
        for it in 0..actual_reps {
            for index in 0..np {
                let i = if it % 2 == 0 { index } else { np - 1 - index };
                if i > 0 || it % 2 == 0 {
                    let a = section_angle * it as f32
                        + if it % 2 == 0 {
                            angles[i]
                        } else {
                            section_angle - angles[i] + 2.0 * angles[0]
                        };
                    let radians = a.to_radians();
                    out.push((
                        (
                            center.0 + radians.cos() * distances[i],
                            center.1 + radians.sin() * distances[i],
                        ),
                        points[i].1,
                    ));
                }
            }
        }
        out
    } else {
        let mut out = Vec::with_capacity(np * reps);
        for i in 0..np * reps {
            let (point, rounding) = points[i % np];
            let radians = ((i / np) as f32 * 360.0 / reps as f32).to_radians();
            let dx = point.0 - center.0;
            let dy = point.1 - center.1;
            out.push((
                (
                    center.0 + dx * radians.cos() - dy * radians.sin(),
                    center.1 + dx * radians.sin() + dy * radians.cos(),
                ),
                rounding,
            ));
        }
        out
    }
}

fn custom_polygon(
    points: &[((f32, f32), CornerRounding)],
    reps: usize,
    center: (f32, f32),
    mirroring: bool,
) -> RoundedPolygon {
    let actual = repeat_points(points, reps, center, mirroring);
    let vertices: Vec<(f32, f32)> = actual.iter().map(|(point, _)| *point).collect();
    let rounding: Vec<CornerRounding> = actual.iter().map(|(_, rounding)| *rounding).collect();
    RoundedPolygon::from_vertices(
        &vertices,
        CornerRounding::UNROUNDED,
        Some(&rounding),
        Some(center),
    )
}

fn r(x: f32, y: f32, radius: f32) -> ((f32, f32), CornerRounding) {
    ((x, y), CornerRounding::new(radius))
}

fn indeterminate_polygons() -> Vec<RoundedPolygon> {
    let soft_burst = custom_polygon(
        &[r(0.193, 0.277, 0.053), r(0.176, 0.055, 0.053)],
        10,
        (0.5, 0.5),
        false,
    );
    let cookie9 = crate::shape::star_polygon(9, 1.0, 0.8, CornerRounding::new(0.5))
        .transformed(|x, y| rotate_about(x, y, -90.0, (0.0, 0.0)));
    let pentagon = custom_polygon(
        &[
            r(0.5, -0.009, 0.172),
            r(1.03, 0.365, 0.164),
            r(0.828, 0.97, 0.169),
        ],
        1,
        (0.5, 0.5),
        true,
    );
    let pill = custom_polygon(
        &[
            r(0.961, 0.039, 0.426),
            r(1.001, 0.428, 0.0),
            r(1.0, 0.609, 1.0),
        ],
        2,
        (0.5, 0.5),
        true,
    );
    let sunny = crate::shape::star_polygon(8, 1.0, 0.8, CornerRounding::new(0.15));
    let cookie4 = custom_polygon(
        &[r(1.237, 1.236, 0.258), r(0.5, 0.918, 0.233)],
        4,
        (0.5, 0.5),
        false,
    );
    let oval = crate::shape::circle_polygon(8, 1.0)
        .transformed(|x, y| (x, y * 0.64))
        .transformed(|x, y| rotate_about(x, y, -45.0, (0.0, 0.0)));

    vec![soft_burst, cookie9, pentagon, pill, sunny, cookie4, oval]
        .into_iter()
        .map(|polygon| polygon.normalized())
        .collect()
}

fn determinate_polygons() -> Vec<RoundedPolygon> {
    let circle = crate::shape::circle_polygon(8, 1.0)
        .transformed(|x, y| rotate_about(x, y, 18.0, (0.0, 0.0)));
    let soft_burst = indeterminate_polygons().remove(0);
    vec![circle, soft_burst]
        .into_iter()
        .map(|polygon| polygon.normalized())
        .collect()
}

fn build_shape_set(polygons: Vec<RoundedPolygon>, circular: bool) -> ShapeSet {
    let scale_factor = sequence_scale(&polygons);
    let mut morphs = Vec::new();
    let mut centers = Vec::new();
    for i in 0..polygons.len() {
        let end_index = match i + 1 < polygons.len() {
            true => i + 1,
            false if circular => 0,
            false => break,
        };
        centers.push((polygons[i].center(), polygons[end_index].center()));
        morphs.push(Morph::new(&polygons[i], &polygons[end_index]));
    }
    ShapeSet {
        morphs,
        centers,
        scale_factor,
    }
}

fn shape_set() -> &'static (ShapeSet, ShapeSet) {
    static SHAPES: std::sync::OnceLock<(ShapeSet, ShapeSet)> = std::sync::OnceLock::new();
    SHAPES.get_or_init(|| {
        let indeterminate = build_shape_set(indeterminate_polygons(), true);
        let determinate = build_shape_set(determinate_polygons(), false);
        (indeterminate, determinate)
    })
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

struct Frame {
    cubics: Vec<Cubic>,
    morph_center: (f32, f32),
    extra_scale: f32,
    rotation_turns: f32,
}

struct AnimatedIndicatorElement {
    id: ElementId,
    elapsed: Rc<Cell<Option<Duration>>>,
    child: Option<AnyElement>,
}

impl IntoElement for AnimatedIndicatorElement {
    type Element = Self;

    fn into_element(self) -> Self {
        self
    }
}

impl Element for AnimatedIndicatorElement {
    type RequestLayoutState = AnyElement;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let start = window.with_element_state(
            global_id.expect("indicator rendered outside of a draw pass"),
            |state: Option<Instant>, _window| {
                let start = state.unwrap_or_else(Instant::now);
                (start, start)
            },
        );
        self.elapsed.set(Some(start.elapsed()));
        window.request_animation_frame();
        let mut child = self.child.take().expect("indicator child taken twice");
        let layout_id = child.request_layout(window, cx);
        (layout_id, child)
    }

    fn prepaint(
        &mut self,
        _global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: gpui::Bounds<Pixels>,
        child: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        child.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: gpui::Bounds<Pixels>,
        child: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        child.paint(window, cx);
    }
}

fn paint_frame(
    window: &mut Window,
    frame: &Frame,
    bounds_center: Point<Pixels>,
    indicator_size: f32,
    sequence_scale: f32,
    color: Hsla,
) {
    let scale = indicator_size * sequence_scale * frame.extra_scale;
    let morph_center = (frame.morph_center.0 * scale, frame.morph_center.1 * scale);
    let offset = (
        f32::from(bounds_center.x) - morph_center.0,
        f32::from(bounds_center.y) - morph_center.1,
    );
    let rotation = frame.rotation_turns * TAU;
    let (sin, cos) = rotation.sin_cos();

    let transform = |x: f32, y: f32| -> (f32, f32) {
        let x = x * scale + offset.0;
        let y = y * scale + offset.1;
        let dx = x - f32::from(bounds_center.x);
        let dy = y - f32::from(bounds_center.y);
        (
            f32::from(bounds_center.x) + dx * cos - dy * sin,
            f32::from(bounds_center.y) + dx * sin + dy * cos,
        )
    };

    let mut builder = PathBuilder::fill();
    for (i, cubic) in frame.cubics.iter().enumerate() {
        let (a0x, a0y) = transform(cubic.anchor0_x, cubic.anchor0_y);

        if i == 0 {
            builder.move_to(point(px(a0x), px(a0y)));
        } else {
            builder.line_to(point(px(a0x), px(a0y)));
        }
        let (c0x, c0y) = transform(cubic.control0_x, cubic.control0_y);
        let (c1x, c1y) = transform(cubic.control1_x, cubic.control1_y);
        let (a1x, a1y) = transform(cubic.anchor1_x, cubic.anchor1_y);
        builder.cubic_bezier_to(
            point(px(a1x), px(a1y)),
            point(px(c0x), px(c0y)),
            point(px(c1x), px(c1y)),
        );
    }
    builder.close();
    if let Ok(path) = builder.build() {
        window.paint_path(path, color);
    }
}

impl LoadingIndicator {
    fn colors(&self, cx: &App) -> (Hsla, Hsla) {
        let tokens = cx.theme().token_set();
        match self.variant {
            LoadingIndicatorVariant::Default => (
                crate::tokens::LoadingIndicatorTokens::ACTIVE_INDICATOR_COLOR.resolve(tokens),
                gpui::transparent_black(),
            ),
            LoadingIndicatorVariant::Contained => (
                crate::tokens::LoadingIndicatorTokens::CONTAINED_ACTIVE_COLOR.resolve(tokens),
                crate::tokens::LoadingIndicatorTokens::CONTAINED_CONTAINER_COLOR.resolve(tokens),
            ),
        }
    }
}

impl gpui::RenderOnce for LoadingIndicator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        use gpui::Styled as _;
        let (indicator_color, container_color) = self.colors(cx);
        let sequence: &'static (ShapeSet, ShapeSet) = shape_set();
        let indicator_color = Cell::new(indicator_color);

        let indicator = match self.progress {
            Some(progress) => {
                let set = &sequence.1;
                let index =
                    (((set.morphs.len() as f32) * progress) as usize).min(set.morphs.len() - 1);
                let adjusted = if progress >= 1.0 && index == set.morphs.len() - 1 {
                    1.0
                } else {
                    (progress * set.morphs.len() as f32) % 1.0
                };
                let cubics = set.morphs[index].as_cubics(adjusted);
                let (start_center, end_center) = set.centers[index];
                let frame = Frame {
                    cubics,
                    morph_center: (
                        lerp(start_center.0, end_center.0, adjusted),
                        lerp(start_center.1, end_center.1, adjusted),
                    ),
                    extra_scale: 1.0,
                    rotation_turns: -progress * 0.5,
                };
                canvas(
                    move |_, _, _| frame,
                    move |bounds, frame, window, _| {
                        paint_frame(
                            window,
                            &frame,
                            bounds.center(),
                            f32::from(self.indicator_size),
                            set.scale_factor,
                            indicator_color.get(),
                        );
                    },
                )
                .size_full()
                .into_any_element()
            }
            None => {
                let motion_spec = *cx.theme().motion().spec(MotionRole::DefaultSpatial);
                let indicator_size = f32::from(self.indicator_size);
                let elapsed_cell = Rc::new(Cell::new(None::<Duration>));
                let paint_cell = elapsed_cell.clone();

                let canvas_element = canvas(
                    move |_, _, _| (),
                    move |bounds, (), window, _| {
                        let Some(elapsed) = paint_cell.get() else {
                            return;
                        };
                        let elapsed_ms = elapsed.as_millis() as f32;
                        let set = &sequence.0;
                        let segment_count = set.morphs.len();
                        let segment_index =
                            ((elapsed_ms / MORPH_INTERVAL_MILLIS as f32) as usize) % segment_count;
                        let fraction = (elapsed_ms % MORPH_INTERVAL_MILLIS as f32)
                            / MORPH_INTERVAL_MILLIS as f32;
                        let active_progress = (fraction / MORPH_ACTIVE_FRACTION).clamp(0.0, 1.0);
                        let eased = motion_spec
                            .fallback_easing
                            .sample(f64::from(active_progress))
                            as f32;
                        let morph_progress = eased.clamp(0.0, 1.0);
                        let cubics = set.morphs[segment_index].as_cubics(morph_progress);
                        let (start_center, end_center) = set.centers[segment_index];
                        let frame = Frame {
                            cubics,
                            morph_center: (
                                lerp(start_center.0, end_center.0, morph_progress),
                                lerp(start_center.1, end_center.1, morph_progress),
                            ),
                            extra_scale: 1.0
                                + MORPH_SCALE_AMPLITUDE
                                    * (std::f32::consts::PI * morph_progress).sin().powi(2),
                            rotation_turns: morph_progress * QUARTER_TURN
                                + QUARTER_TURN * (segment_index as f32 + 1.0)
                                + (elapsed_ms % GLOBAL_ROTATION_MILLIS as f32)
                                    / GLOBAL_ROTATION_MILLIS as f32,
                        };
                        paint_frame(
                            window,
                            &frame,
                            bounds.center(),
                            indicator_size,
                            set.scale_factor,
                            indicator_color.get(),
                        );
                    },
                )
                .size_full()
                .into_any_element();

                AnimatedIndicatorElement {
                    id: self.id.clone(),
                    elapsed: elapsed_cell,
                    child: Some(canvas_element),
                }
                .into_any_element()
            }
        };

        div()
            .id(self.id.clone())
            .size(self.size)
            .flex_none()
            .rounded_full()
            .overflow_hidden()
            .bg(container_color)
            .child(indicator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::Morph;

    fn assert_closed(cubics: &[Cubic], tolerance: f32, context: &str) {
        assert!(cubics.len() >= 2, "{context}: too few cubics");
        for i in 0..cubics.len() {
            let current = &cubics[i];
            let next = &cubics[(i + 1) % cubics.len()];
            let dx = current.anchor1_x - next.anchor0_x;
            let dy = current.anchor1_y - next.anchor0_y;
            assert!(
                dx.hypot(dy) <= tolerance,
                "{context}: cubic {i} ends at ({}, {}) but next starts at ({}, {})",
                current.anchor1_x,
                current.anchor1_y,
                next.anchor0_x,
                next.anchor0_y
            );
        }
    }

    fn assert_within(cubics: &[Cubic], bounds: (f32, f32, f32, f32), slack: f32, context: &str) {
        let (x0, y0, x1, y1) = bounds;
        for (i, cubic) in cubics.iter().enumerate() {
            let points = [
                (cubic.anchor0_x, cubic.anchor0_y),
                (cubic.control0_x, cubic.control0_y),
                (cubic.control1_x, cubic.control1_y),
                (cubic.anchor1_x, cubic.anchor1_y),
            ];
            for (x, y) in points {
                assert!(
                    x >= x0 - slack && x <= x1 + slack && y >= y0 - slack && y <= y1 + slack,
                    "{context}: cubic {i} point ({x}, {y}) outside bounds ({x0}, {y0})-({x1}, {y1})"
                );
            }
        }
    }

    #[test]
    fn morph_frames_stay_closed_and_within_union_bounds() {
        let polygons = indeterminate_polygons();
        for i in 0..polygons.len() {
            let start = &polygons[i];
            let end = &polygons[(i + 1) % polygons.len()];
            let morph = Morph::new(start, end);
            let (bx0, by0, bx1, by1) = morph.calculate_bounds();
            let (mx0, my0, mx1, my1) = morph.calculate_max_bounds();
            let outer = (mx0.min(bx0), my0.min(by0), mx1.max(bx1), my1.max(by1));
            for step in 0..=20 {
                let progress = step as f32 / 20.0;
                let context = format!("pair {i} progress {progress}");
                let cubics = morph.as_cubics(progress);
                assert_closed(&cubics, 0.01, &context);
                assert_within(&cubics, outer, 0.03, &context);
            }
        }
    }
}
