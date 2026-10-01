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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/FloatingActionButtonMenu.kt
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/FloatingToolbar.kt

use std::cell::Cell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use gpui::{
    Animation, AnimationExt, AnyElement, App, AppContext as _, Context, ElementId, Entity, Hsla,
    IntoElement, ParentElement as _, PathBuilder, Pixels, Point, Render, RenderOnce, Styled,
    WeakEntity, Window, canvas, div, point, prelude::FluentBuilder as _, prelude::*, px,
};

use crate::components::{Fab, FabState, FilledIconToggleButton};
use crate::icon::IconName;
use crate::motion::{Animatable, AnimatedComponent, AnimationDriver, MotionRole, easing};
use crate::theme::ActiveTheme;

const TAU: f32 = std::f32::consts::TAU;

const LOADING_SHAPE_COUNT: usize = 7;
const LOADING_SEGMENT_MILLIS: u64 = 650;
const LOADING_MORPH_ACTIVE_FRACTION: f32 = 0.72;
const LOADING_BREATHING_AMPLITUDE: f32 = 0.12;
const LINEAR_WAVE_PHASE_MILLIS: u64 = 1000;
const LINEAR_INDETERMINATE_MILLIS: u64 = 1750;
const CIRCULAR_INDETERMINATE_MILLIS: u64 = 6000;

const SHAPE_SAMPLES: usize = 72;

type SelectHandler = Rc<dyn Fn(usize, &mut Window, &mut App)>;

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
    let mut builder = PathBuilder::fill();
    for i in 0..SHAPE_SAMPLES {
        let theta = TAU * i as f32 / SHAPE_SAMPLES as f32;
        let interpolated = from[i] + (to[i] - from[i]) * morph;
        let angle = theta + rotation;
        let (sin, cos) = angle.sin_cos();
        let point = point(
            center.x + px(interpolated * scale * cos),
            center.y + px(interpolated * scale * sin),
        );
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
                let (start, end) = match value {
                    Some(v) => (0.0, width * v),
                    None => {
                        let eased = |t: f32| t.clamp(0.0, 1.0) * t.clamp(0.0, 1.0);
                        let head = eased(phase * 1.25);
                        let tail = eased(phase * 1.25 - 0.25);
                        (width * tail, width * head)
                    }
                };
                if end - start < 2.0 {
                    return;
                }
                let amplitude_scale = match value {
                    Some(v) => (v / 0.1).min(1.0) * ((1.0 - v) / 0.05).min(1.0),
                    None => 1.0,
                };
                let amp = f32::from(amplitude) * amplitude_scale;
                let mut builder = PathBuilder::fill();
                let mut top = Vec::with_capacity(((end - start) / 3.0) as usize + 2);
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
                for (_, y) in top.iter().rev() {
                    builder.line_to(point(px(0.), *y + thickness));
                }
                for (x, y) in top.iter().rev() {
                    builder.line_to(point(px(*x), *y + thickness));
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
            .relative()
            .w_full()
            .flex_none()
            .h(wave_height)
            .child(
                div()
                    .absolute()
                    .top((wave_height - thickness) * 0.5)
                    .left_0()
                    .right_0()
                    .h(thickness)
                    .rounded_full()
                    .bg(track_color),
            )
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

pub struct FabMenu {
    id: ElementId,
    expanded: bool,
    items: Vec<Entity<FabState>>,
}

pub type FloatingActionButtonMenu = FabMenu;
pub type FloatingActionButtonMenuItem = Fab;
pub type ToggleFloatingActionButton = FilledIconToggleButton;

impl FabMenu {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            expanded: false,
            items: Vec::new(),
        }
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    pub fn action(mut self, item: Entity<FabState>) -> Self {
        self.items.push(item);
        self
    }

    pub fn build(self, cx: &mut App) -> Entity<FabMenuState> {
        cx.new(|cx| {
            let weak: WeakEntity<FabMenuState> = cx.entity().downgrade();
            let toggle_add = Fab::new((self.id.clone(), "toggle-add"), IconName::Add)
                .on_click({
                    let weak = weak.clone();
                    move |_, window, cx| {
                        if let Err(err) =
                            weak.update(cx, |menu, cx| menu.set_expanded(true, window, cx))
                        {
                            eprintln!("material3-gpui: {err}");
                        }
                    }
                })
                .build(cx);
            let toggle_close = Fab::new((self.id.clone(), "toggle-close"), IconName::Close)
                .on_click(move |_, window, cx| {
                    if let Err(err) =
                        weak.update(cx, |menu, cx| menu.set_expanded(false, window, cx))
                    {
                        eprintln!("material3-gpui: {err}");
                    }
                })
                .build(cx);
            FabMenuState {
                id: self.id,
                expanded: self.expanded,
                progress: Animatable::new(if self.expanded { 1.0 } else { 0.0 }, 1.0e-3),
                items: self.items,
                toggle_add,
                toggle_close,
                driver: AnimationDriver::default(),
            }
        })
    }
}

pub struct FabMenuState {
    id: ElementId,
    expanded: bool,
    progress: Animatable,
    items: Vec<Entity<FabState>>,
    toggle_add: Entity<FabState>,
    toggle_close: Entity<FabState>,
    driver: AnimationDriver,
}

impl FabMenuState {
    pub fn expanded(&self) -> bool {
        self.expanded
    }

    pub fn set_expanded(&mut self, expanded: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.expanded == expanded {
            return;
        }
        self.expanded = expanded;
        let role = if expanded {
            MotionRole::DefaultSpatial
        } else {
            MotionRole::FastSpatial
        };
        let spec = *cx.theme().motion().spec(role);
        self.progress
            .animate_to(if expanded { 1.0 } else { 0.0 }, &spec, Instant::now());
        if self.progress.is_running() {
            self.schedule_next(window, cx);
        }
        cx.notify();
    }
}

impl AnimatedComponent for FabMenuState {
    fn step(&mut self, now: Instant) -> bool {
        self.progress.tick(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }
}

impl Render for FabMenuState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.progress.is_running() {
            self.schedule_next(window, cx);
        }
        let progress = self.progress.value() as f32;
        let item_count = self.items.len();

        let mut column = div().flex().flex_col().items_end().gap(px(4.));
        for (index, item) in self.items.iter().enumerate() {
            let sink = px(16.0 * (item_count - index) as f32 * (1.0 - progress));
            column = column.child(
                div()
                    .relative()
                    .top(sink)
                    .opacity(progress)
                    .child(item.clone()),
            );
        }

        let toggle = if self.expanded {
            self.toggle_close.clone()
        } else {
            self.toggle_add.clone()
        };

        div()
            .id(self.id.clone())
            .flex()
            .flex_col()
            .items_end()
            .gap(px(8.))
            .child(column)
            .child(toggle)
    }
}

#[derive(IntoElement)]
pub struct FloatingToolbar {
    id: ElementId,
    children: Vec<AnyElement>,
    vertical: bool,
    vibrant: bool,
    state: FloatingToolbarState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FloatingToolbarState {
    pub expanded: bool,
}

impl FloatingToolbarState {
    pub fn new(expanded: bool) -> Self {
        Self { expanded }
    }

    pub fn expand(&mut self) {
        self.expanded = true;
    }

    pub fn collapse(&mut self) {
        self.expanded = false;
    }
}

impl Default for FloatingToolbarState {
    fn default() -> Self {
        Self::new(true)
    }
}

pub type HorizontalFloatingToolbar = FloatingToolbar;

#[derive(IntoElement)]
pub struct VerticalFloatingToolbar(FloatingToolbar);

impl VerticalFloatingToolbar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self(FloatingToolbar::new(id).vertical())
    }

    pub fn state(mut self, state: FloatingToolbarState) -> Self {
        self.0 = self.0.state(state);
        self
    }

    pub fn vibrant(mut self, vibrant: bool) -> Self {
        self.0 = self.0.vibrant(vibrant);
        self
    }
}

impl ParentElement for VerticalFloatingToolbar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.0.extend(elements);
    }
}

impl RenderOnce for VerticalFloatingToolbar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.0.render(window, cx)
    }
}

impl FloatingToolbar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: Vec::new(),
            vertical: false,
            vibrant: false,
            state: FloatingToolbarState::default(),
        }
    }

    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }

    pub fn vibrant(mut self, vibrant: bool) -> Self {
        self.vibrant = vibrant;
        self
    }

    pub fn state(mut self, state: FloatingToolbarState) -> Self {
        self.state = state;
        self
    }
}

impl ParentElement for FloatingToolbar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for FloatingToolbar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        use crate::tokens::FloatingToolbarTokens as Tokens;
        let theme = cx.theme();
        let tokens = theme.token_set();
        let container_color = if self.vibrant {
            Tokens::VIBRANT_CONTAINER_COLOR.resolve(tokens)
        } else {
            Tokens::STANDARD_CONTAINER_COLOR.resolve(tokens)
        };
        let content_color = if self.vibrant {
            theme.colors().on_primary_container
        } else {
            theme.colors().on_surface_variant
        };
        let slot = px(48.);
        let container = if self.vertical {
            div().w(px(f32::from(slot) + 16.0)).h_auto()
        } else {
            div().h(Tokens::CONTAINER_HEIGHT.pixels())
        };
        container
            .id(self.id)
            .flex_none()
            .flex()
            .when(self.vertical, |el| el.flex_col())
            .items_center()
            .justify_center()
            .gap(Tokens::CONTAINER_BETWEEN_SPACE.pixels())
            .p(px(8.))
            .rounded_full()
            .bg(container_color)
            .text_color(content_color)
            .shadow_lg()
            .when(self.state.expanded, |el| {
                el.children(self.children.into_iter().map(|child| {
                    div()
                        .size(slot)
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(child)
                }))
            })
    }
}

#[derive(IntoElement)]
pub struct Carousel {
    id: ElementId,
    items: Vec<AnyElement>,
    selected: usize,
    on_select: Option<SelectHandler>,
}

impl Carousel {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            selected: 0,
            on_select: None,
        }
    }

    pub fn selected(mut self, selected: usize) -> Self {
        self.selected = selected;
        self
    }

    pub fn on_select(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
}

impl ParentElement for Carousel {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.items.extend(elements);
    }
}

impl RenderOnce for Carousel {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = cx.theme().colors();
        let selected = self.selected.min(self.items.len().saturating_sub(1));
        div()
            .id(self.id)
            .w_full()
            .h(px(232.))
            .flex_none()
            .flex()
            .gap(px(8.))
            .children(self.items.into_iter().enumerate().map(|(index, item)| {
                let is_selected = index == selected;
                let is_neighbor = (index as i64 - selected as i64).abs() == 1;
                let handler = self.on_select.clone();
                div()
                    .id(("md3-carousel-item", index))
                    .h_full()
                    .flex_none()
                    .rounded(px(28.))
                    .overflow_hidden()
                    .bg(colors.surface_container_high)
                    .when(is_selected, |el| el.flex_1().min_w_0())
                    .when(!is_selected, |el| {
                        el.w(px(if is_neighbor { 120. } else { 56. }))
                    })
                    .when_some(handler, |el, handler| {
                        el.cursor_pointer().on_click(move |_, window, cx| {
                            handler(index, window, cx);
                        })
                    })
                    .child(
                        div()
                            .size_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(item),
                    )
            }))
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
