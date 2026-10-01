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
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/skins/M3StateLayer.java
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/skins/M3LabeledButtonSkinBase.java

use std::cell::RefCell;
use std::f32::consts::{FRAC_PI_2, PI};
use std::rc::Rc;
use std::time::Instant;

use gpui::{
    AnyElement, Bounds, Canvas, Corners, Div, Entity, Hsla, InteractiveElement as _,
    IntoElement as _, MouseButton, ParentElement as _, PathBuilder, Pixels, Point, Stateful,
    StatefulInteractiveElement as _, Styled, canvas, div, point, px,
};

use crate::motion::{Animatable, AnimationDriver, MotionRole, MotionScheme, lerp_color};

const HOVER_PROGRESS: f64 = 0.8;

#[derive(Clone, Debug, Default)]
pub struct BoundsHandle(Rc<RefCell<Bounds<Pixels>>>);

impl BoundsHandle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self) -> Bounds<Pixels> {
        *self.0.borrow()
    }

    pub fn capture_element(&self) -> Canvas<()> {
        let handle = self.0.clone();
        canvas(
            move |bounds, _window, _cx| {
                *handle.borrow_mut() = bounds;
            },
            |_, _, _, _| {},
        )
        .absolute()
        .inset_0()
    }
}

#[derive(Clone, Debug)]
pub struct InteractiveSurface {
    pub bounds: BoundsHandle,

    pub hovered: bool,

    pub pressed: bool,

    pub ripple_enabled: bool,

    ripple_max_radius: Option<Pixels>,
    state_layer_opacity: Animatable,
    ripple: Option<Ripple>,
    driver: AnimationDriver,
}

#[derive(Clone, Copy, Debug)]
struct Ripple {
    origin: Point<Pixels>,

    radius: Animatable,

    fade: Animatable,
}

impl Default for InteractiveSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl InteractiveSurface {
    pub fn new() -> Self {
        Self {
            bounds: BoundsHandle::new(),
            hovered: false,
            pressed: false,
            ripple_enabled: true,
            ripple_max_radius: None,
            state_layer_opacity: Animatable::new(0.0, 1.0e-3),
            ripple: None,
            driver: AnimationDriver::default(),
        }
    }

    pub fn set_ripple_max_radius(&mut self, max_radius: Option<Pixels>) {
        self.ripple_max_radius = max_radius;
    }

    pub fn hover_progress(&self) -> f64 {
        f64::clamp(self.state_layer_opacity.value() / HOVER_PROGRESS, 0.0, 1.0)
    }

    pub fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }

    pub fn step(&mut self, now: Instant) -> bool {
        self.state_layer_opacity.tick(now);
        let mut ripple_done = false;
        if let Some(ripple) = &mut self.ripple {
            let radius_running = ripple.radius.tick(now);
            let fade_running = ripple.fade.tick(now);
            if !radius_running && !fade_running && ripple.fade.value() <= 0.0 {
                ripple_done = true;
            }
        }
        if ripple_done {
            self.ripple = None;
        }
        self.is_animating()
    }

    pub fn is_animating(&self) -> bool {
        self.state_layer_opacity.is_running()
            || self.ripple.as_ref().is_some_and(|r| {
                r.radius.is_running() || r.fade.is_running() || r.fade.value() > 0.0
            })
    }

    pub fn set_hovered(&mut self, hovered: bool, motion: &MotionScheme, now: Instant) {
        self.hovered = hovered;
        if !hovered {
            self.pressed = false;
        }
        self.animate_state_layer_to(self.target_opacity(), motion, now);
    }

    pub fn on_press(&mut self, position: Point<Pixels>, motion: &MotionScheme, now: Instant) {
        self.pressed = true;
        self.state_layer_opacity
            .animate_to(1.0, motion.spec(MotionRole::FastEffects), now);

        if !self.ripple_enabled {
            return;
        }

        let bounds = self.bounds.get();
        let local_x = f32::from(position.x - bounds.origin.x);
        let local_y = f32::from(position.y - bounds.origin.y);
        let width = f32::from(bounds.size.width);
        let height = f32::from(bounds.size.height);
        let dx2 = |x: f32, y: f32| f64::from(x * x + y * y);
        let auto_radius = f64::max(
            f64::max(dx2(local_x, local_y), dx2(width - local_x, local_y)),
            f64::max(
                dx2(local_x, height - local_y),
                dx2(width - local_x, height - local_y),
            ),
        )
        .sqrt();
        let max_radius = self.ripple_max_radius.map(f64::from).unwrap_or(auto_radius);
        let mut ripple = Ripple {
            origin: point(px(local_x), px(local_y)),
            radius: Animatable::new(0.0, 1.0e-2),
            fade: Animatable::new(1.0, 1.0e-2),
        };

        ripple
            .radius
            .animate_to(max_radius, motion.spec(MotionRole::DefaultSpatial), now);
        self.ripple = Some(ripple);
    }

    pub fn on_release(&mut self, motion: &MotionScheme, now: Instant) {
        self.pressed = false;
        self.animate_state_layer_to(self.target_opacity(), motion, now);
        self.fade_ripple(motion, now);
    }

    pub fn on_cancel(&mut self, motion: &MotionScheme, now: Instant) {
        self.pressed = false;
        self.animate_state_layer_to(self.target_opacity(), motion, now);
        self.fade_ripple(motion, now);
    }

    fn fade_ripple(&mut self, motion: &MotionScheme, now: Instant) {
        if let Some(ripple) = &mut self.ripple {
            ripple
                .fade
                .animate_to(0.0, motion.spec(MotionRole::DefaultEffects), now);
        }
    }

    fn target_opacity(&self) -> f64 {
        if self.pressed {
            1.0
        } else if self.hovered {
            HOVER_PROGRESS
        } else {
            0.0
        }
    }

    fn animate_state_layer_to(&mut self, target: f64, motion: &MotionScheme, now: Instant) {
        let role = if target > self.state_layer_opacity.value() {
            MotionRole::FastEffects
        } else {
            MotionRole::DefaultEffects
        };
        self.state_layer_opacity
            .animate_to(target, motion.spec(role), now);
    }

    pub fn overlay(
        &self,
        base_color: Hsla,
        pressed_opacity: f32,
        corner_radius: Pixels,
    ) -> InteractiveOverlay {
        self.overlay_with_corners(base_color, pressed_opacity, Corners::all(corner_radius))
    }

    pub fn overlay_with_corners(
        &self,
        base_color: Hsla,
        pressed_opacity: f32,
        corner_radius: Corners<Pixels>,
    ) -> InteractiveOverlay {
        let layer_alpha = f32::clamp(
            self.state_layer_opacity.value() as f32 * pressed_opacity,
            0.0,
            1.0,
        );
        InteractiveOverlay {
            state_layer_color: if layer_alpha > 0.0 {
                Some(lerp_color(
                    Hsla::transparent_black(),
                    base_color,
                    layer_alpha,
                ))
            } else {
                None
            },
            ripple: self.ripple_elements(base_color, pressed_opacity, corner_radius, false),
            corner_radius,
        }
    }

    pub fn overlay_unclipped(&self, base_color: Hsla, ripple_opacity: f32) -> InteractiveOverlay {
        InteractiveOverlay {
            state_layer_color: None,
            ripple: self.ripple_elements(base_color, ripple_opacity, Corners::all(px(0.)), true),
            corner_radius: Corners::all(px(0.)),
        }
    }

    pub(crate) fn set_ripple_origin(&mut self, origin: Point<Pixels>) {
        if let Some(ripple) = &mut self.ripple {
            ripple.origin = origin;
        }
    }

    fn ripple_elements(
        &self,
        base_color: Hsla,
        ripple_opacity: f32,
        corner_radius: Corners<Pixels>,
        unclipped: bool,
    ) -> Option<AnyElement> {
        let ripple = self.ripple.as_ref()?;
        if ripple.fade.value() <= 0.0 || ripple.radius.value() <= 0.0 {
            return None;
        }
        let radius = ripple.radius.value() as f32;
        let alpha = f32::clamp(ripple.fade.value() as f32 * ripple_opacity, 0.0, 1.0);
        let color = lerp_color(Hsla::transparent_black(), base_color, alpha);
        Some(ripple_element(
            ripple.origin,
            radius,
            color,
            corner_radius,
            unclipped,
        ))
    }
}

pub struct InteractiveOverlay {
    state_layer_color: Option<Hsla>,

    ripple: Option<AnyElement>,

    corner_radius: Corners<Pixels>,
}

impl InteractiveOverlay {
    pub fn apply(self, container: Stateful<Div>) -> Stateful<Div> {
        let mut container = container;
        if let Some(color) = self.state_layer_color {
            container = container.child(
                div()
                    .absolute()
                    .inset_0()
                    .rounded_tl(self.corner_radius.top_left)
                    .rounded_tr(self.corner_radius.top_right)
                    .rounded_bl(self.corner_radius.bottom_left)
                    .rounded_br(self.corner_radius.bottom_right)
                    .bg(color),
            );
        }
        if let Some(ripple) = self.ripple {
            container = container.child(ripple);
        }
        container
    }
}

fn ripple_element(
    origin: Point<Pixels>,
    radius: f32,
    color: Hsla,
    corner_radius: Corners<Pixels>,
    unclipped: bool,
) -> AnyElement {
    canvas(
        |_, _, _| {},
        move |bounds, _, window, _| {
            let center = bounds.origin + origin;
            let circle = circle_polygon(center, radius);

            let points = if unclipped
                || (corner_radius == Corners::all(corner_radius.top_left)
                    && circle_inside_rounded_rect(bounds, corner_radius.top_left, center, radius))
            {
                circle
            } else {
                let clip = rounded_rect_polygon(bounds, corner_radius);
                let Some(points) = clip_convex(&circle, &clip) else {
                    return;
                };
                points
            };
            let mut builder = PathBuilder::fill();
            builder.add_polygon(&points, true);
            if let Ok(path) = builder.build() {
                window.paint_path(path, color);
            }
        },
    )
    .absolute()
    .inset_0()
    .into_any_element()
}

fn circle_inside_rounded_rect(
    bounds: Bounds<Pixels>,
    corner_radius: Pixels,
    center: Point<Pixels>,
    radius: f32,
) -> bool {
    let w = f32::from(bounds.size.width);
    let h = f32::from(bounds.size.height);
    let r = f32::from(corner_radius).clamp(0.0, w.min(h) / 2.0);
    let (x0, y0) = (f32::from(bounds.origin.x), f32::from(bounds.origin.y));
    let (x1, y1) = (x0 + w, y0 + h);
    let (cx, cy) = (f32::from(center.x), f32::from(center.y));

    if cx - radius < x0 || cx + radius > x1 || cy - radius < y0 || cy + radius > y1 {
        return false;
    }

    let corner_ok = |ccx: f32, ccy: f32, outside_x: bool, outside_y: bool| {
        if !(outside_x && outside_y) {
            return true;
        }
        let (dx, dy) = (cx - ccx, cy - ccy);
        (dx * dx + dy * dy).sqrt() + radius <= r
    };
    corner_ok(x0 + r, y0 + r, cx < x0 + r, cy < y0 + r)
        && corner_ok(x1 - r, y0 + r, cx > x1 - r, cy < y0 + r)
        && corner_ok(x0 + r, y1 - r, cx < x0 + r, cy > y1 - r)
        && corner_ok(x1 - r, y1 - r, cx > x1 - r, cy > y1 - r)
}

fn circle_polygon(center: Point<Pixels>, radius: f32) -> Vec<Point<Pixels>> {
    const CIRCLE_SEGMENTS: usize = 32;
    (0..CIRCLE_SEGMENTS)
        .map(|i| {
            let angle = std::f32::consts::TAU * i as f32 / CIRCLE_SEGMENTS as f32;
            point(
                center.x + px(radius * angle.cos()),
                center.y + px(radius * angle.sin()),
            )
        })
        .collect()
}

fn rounded_rect_polygon(
    bounds: Bounds<Pixels>,
    corner_radius: impl Into<Corners<Pixels>>,
) -> Vec<Point<Pixels>> {
    const ARC_SEGMENTS: usize = 6;
    let w = f32::from(bounds.size.width);
    let h = f32::from(bounds.size.height);

    let corners = corner_radius
        .into()
        .map(|radius| f32::from(*radius).clamp(0.0, w.min(h) / 2.0));
    let (x0, y0) = (f32::from(bounds.origin.x), f32::from(bounds.origin.y));
    let (x1, y1) = (x0 + w, y0 + h);
    let mut points = Vec::with_capacity(4 * (ARC_SEGMENTS + 1));

    for (center_x, center_y, radius, start) in [
        (
            x1 - corners.top_right,
            y0 + corners.top_right,
            corners.top_right,
            -FRAC_PI_2,
        ),
        (
            x1 - corners.bottom_right,
            y1 - corners.bottom_right,
            corners.bottom_right,
            0.0,
        ),
        (
            x0 + corners.bottom_left,
            y1 - corners.bottom_left,
            corners.bottom_left,
            FRAC_PI_2,
        ),
        (
            x0 + corners.top_left,
            y0 + corners.top_left,
            corners.top_left,
            PI,
        ),
    ] {
        if radius == 0.0 {
            points.push(point(px(center_x), px(center_y)));
            continue;
        }
        for step in 0..=ARC_SEGMENTS {
            let angle = start + FRAC_PI_2 * step as f32 / ARC_SEGMENTS as f32;
            points.push(point(
                px(center_x + radius * angle.cos()),
                px(center_y + radius * angle.sin()),
            ));
        }
    }
    points
}

fn edge_cross(a: Point<Pixels>, b: Point<Pixels>, p: Point<Pixels>) -> f32 {
    let (ux, uy) = (f32::from(b.x - a.x), f32::from(b.y - a.y));
    let (vx, vy) = (f32::from(p.x - a.x), f32::from(p.y - a.y));
    ux * vy - uy * vx
}

fn clip_convex(subject: &[Point<Pixels>], clip: &[Point<Pixels>]) -> Option<Vec<Point<Pixels>>> {
    let mut output: Vec<Point<Pixels>> = subject.to_vec();
    let mut scratch: Vec<Point<Pixels>> = Vec::with_capacity(subject.len() + clip.len());
    for i in 0..clip.len() {
        if output.is_empty() {
            return None;
        }
        let a = clip[i];
        let b = clip[(i + 1) % clip.len()];
        scratch.clear();
        for j in 0..output.len() {
            let (p, q) = (output[j], output[(j + 1) % output.len()]);
            let (dp, dq) = (edge_cross(a, b, p), edge_cross(a, b, q));
            let (p_in, q_in) = (dp >= 0.0, dq >= 0.0);
            if p_in != q_in {
                let t = dp / (dp - dq);
                scratch.push(point(p.x + (q.x - p.x) * t, p.y + (q.y - p.y) * t));
            }
            if q_in {
                scratch.push(q);
            }
        }
        std::mem::swap(&mut output, &mut scratch);
    }
    (output.len() >= 3).then_some(output)
}

#[allow(clippy::too_many_arguments)]
pub fn wire<T: 'static>(
    surface: &InteractiveSurface,
    el: Stateful<Div>,
    entity: &Entity<T>,
    motion: &MotionScheme,
    access: impl Fn(&mut T) -> &mut InteractiveSurface + Copy + 'static,
    state_layer_color: Hsla,
    pressed_opacity: f32,
    corner_radius: Pixels,
) -> Stateful<Div> {
    let el = wire_events(el, entity, motion, access);
    let el = surface
        .overlay(state_layer_color, pressed_opacity, corner_radius)
        .apply(el);
    el.child(surface.bounds.capture_element())
}

pub fn wire_events<T: 'static>(
    el: Stateful<Div>,
    entity: &Entity<T>,
    motion: &MotionScheme,
    access: impl Fn(&mut T) -> &mut InteractiveSurface + 'static + Copy,
) -> Stateful<Div> {
    let hover_entity = entity.clone();
    let press_entity = entity.clone();
    let release_entity = entity.clone();
    let cancel_entity = entity.clone();
    let motion_hover = *motion;
    let motion_press = *motion;
    let motion_release = *motion;
    let motion_cancel = *motion;
    el.on_hover(move |hovered, _window, cx| {
        hover_entity.update(cx, |state, cx| {
            let now = Instant::now();
            access(state).set_hovered(*hovered, &motion_hover, now);
            cx.notify();
        });
    })
    .on_mouse_down(MouseButton::Left, move |event, _window, cx| {
        press_entity.update(cx, |state, cx| {
            let now = Instant::now();
            let position = event.position;
            access(state).on_press(position, &motion_press, now);
            cx.notify();
        });
    })
    .on_mouse_up(MouseButton::Left, move |_event, _window, cx| {
        release_entity.update(cx, |state, cx| {
            let now = Instant::now();
            access(state).on_release(&motion_release, now);
            cx.notify();
        });
    })
    .on_mouse_up_out(MouseButton::Left, move |_event, _window, cx| {
        cancel_entity.update(cx, |state, cx| {
            let now = Instant::now();
            access(state).on_cancel(&motion_cancel, now);
            cx.notify();
        });
    })
}
