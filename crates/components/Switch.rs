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
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/skins/M3SwitchSkin.java

use std::rc::Rc;
use std::time::Instant;

use gpui::{
    App, AppContext as _, Context, DispatchPhase, ElementId, Entity, Hsla, InteractiveElement as _,
    IntoElement, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, ParentElement as _,
    Pixels, Point, Render, StatefulInteractiveElement as _, Styled, Window, canvas, div, point,
    prelude::FluentBuilder as _, px,
};

use crate::icon::{Icon, IconName};
use crate::interaction::InteractiveSurface;
use crate::motion::{Animatable, AnimatedComponent, AnimationDriver, MotionRole, lerp_color};
use crate::theme::ActiveTheme;

type ChangeHandler = Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>;

const DRAG_THRESHOLD: f32 = 4.0;

const TOUCH_TARGET_HEIGHT: f32 = 48.0;

const PRESSED_HANDLE_SIZE: f32 = 28.0;

const STATE_LAYER_SIZE: f32 = 40.0;

const STATE_LAYER_OPACITY: f32 = 0.08;

pub struct Switch {
    id: ElementId,
    checked: bool,
    disabled: bool,
    check_icon: bool,
    on_change: Option<ChangeHandler>,
}

pub struct SwitchState {
    id: ElementId,
    checked: bool,
    disabled: bool,
    check_icon: bool,
    on_change: Option<ChangeHandler>,

    progress: Animatable,

    press_progress: Animatable,

    pressed_observed: bool,

    pressed: bool,

    dragging: bool,

    drag_start_x: Pixels,

    grab_offset: Pixels,
    surface: InteractiveSurface,
}

impl Switch {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            checked: false,
            disabled: false,
            check_icon: false,
            on_change: None,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn enabled(self, enabled: bool) -> Self {
        self.disabled(!enabled)
    }

    pub fn with_check_icon(mut self, check_icon: bool) -> Self {
        self.check_icon = check_icon;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    pub fn on_checked_change(
        self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change(handler)
    }

    pub fn build(self, cx: &mut App) -> Entity<SwitchState> {
        let checked = self.checked;
        let mut surface = InteractiveSurface::new();

        surface.set_ripple_max_radius(Some(px(20.)));
        cx.new(|_| SwitchState {
            id: self.id,
            checked,
            disabled: self.disabled,
            check_icon: self.check_icon,
            on_change: self.on_change,
            progress: Animatable::new(if checked { 1.0 } else { 0.0 }, 1.0e-3),
            press_progress: Animatable::new(0.0, 1.0e-3),
            pressed_observed: false,
            pressed: false,
            dragging: false,
            drag_start_x: px(0.),
            grab_offset: px(0.),
            surface,
        })
    }
}

impl SwitchState {
    pub fn checked(&self) -> bool {
        self.checked
    }

    pub fn set_on_change(&mut self, handler: impl Fn(bool, &mut Window, &mut App) + 'static) {
        self.on_change = Some(Rc::new(handler));
    }

    pub fn set_checked(&mut self, checked: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.checked == checked {
            return;
        }
        self.checked = checked;
        self.animate_thumb_to(if checked { 1.0 } else { 0.0 }, window, cx);
    }

    fn animate_thumb_to(&mut self, target: f64, window: &mut Window, cx: &mut Context<Self>) {
        let spec = *cx.theme().motion().spec(MotionRole::FastSpatial);
        self.progress.animate_to(target, &spec, Instant::now());
        if self.progress.is_running() {
            self.schedule_next(window, cx);
        }
        cx.notify();
    }

    fn apply_selection(&mut self, select: bool, window: &mut Window, cx: &mut Context<Self>) {
        let changed = select != self.checked;
        self.checked = select;
        self.animate_thumb_to(if select { 1.0 } else { 0.0 }, window, cx);
        if changed && let Some(handler) = self.on_change.clone() {
            handler(select, window, cx);
        }
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.disabled {
            return;
        }
        let now = Instant::now();
        let motion = *cx.theme().motion();
        self.surface.on_press(event.position, &motion, now);

        let bounds = self.surface.bounds.get();
        let local_x = event.position.x - bounds.origin.x;
        let style = SwitchStyle::resolve(cx.theme().token_set(), self.disabled);
        let center_x = thumb_center(style.track_size, self.progress.value() as f32);
        self.pressed = true;
        self.dragging = false;
        self.drag_start_x = event.position.x;
        self.grab_offset = local_x - center_x;
        cx.notify();
    }

    fn drag_move(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        if !self.pressed || self.disabled {
            return;
        }
        if !self.dragging {
            if f32::from(position.x - self.drag_start_x).abs() < DRAG_THRESHOLD {
                return;
            }
            self.dragging = true;
            self.progress.stop();
        }
        let bounds = self.surface.bounds.get();

        let re_armed = if bounds.contains(&position) && !self.surface.pressed {
            self.surface.pressed = true;
            true
        } else {
            false
        };
        let local_x = position.x - bounds.origin.x;
        let handle_center = local_x - self.grab_offset;
        let style = SwitchStyle::resolve(cx.theme().token_set(), self.disabled);
        let travel = f32::from(style.track_size.0 - style.track_size.1);
        let pos = if travel > 0. {
            f32::from(handle_center - style.track_size.1 / 2.) / travel
        } else {
            0.
        };
        let next = f64::from(pos.clamp(0.0, 1.0));

        let moved = (next - self.progress.value()).abs() >= 0.025 / 20.0;
        if moved {
            self.progress.snap_to(next);
        }
        if moved || re_armed {
            cx.notify();
        }
    }

    fn drag_end(&mut self, position: Point<Pixels>, window: &mut Window, cx: &mut Context<Self>) {
        if !self.pressed {
            return;
        }
        self.pressed = false;
        let now = Instant::now();
        let motion = *cx.theme().motion();
        self.surface.on_release(&motion, now);

        let inside = self.surface.bounds.get().contains(&position);
        if self.dragging {
            self.dragging = false;

            self.apply_selection(self.progress.value() >= 0.5, window, cx);
        } else if inside {
            self.apply_selection(!self.checked, window, cx);
        } else {
            self.animate_thumb_to(if self.checked { 1.0 } else { 0.0 }, window, cx);
        }
        cx.notify();
    }

    fn on_mouse_up(&mut self, event: &MouseUpEvent, window: &mut Window, cx: &mut Context<Self>) {
        self.drag_end(event.position, window, cx);
    }
}

fn thumb_center(track_size: (Pixels, Pixels), progress: f32) -> Pixels {
    track_size.1 / 2. + (track_size.0 - track_size.1) * progress.clamp(0., 1.)
}

impl AnimatedComponent for SwitchState {
    fn step(&mut self, now: Instant) -> bool {
        let running = self.progress.tick(now);
        let surface_running = self.surface.step(now);
        let press_running = self.press_progress.tick(now);
        running || surface_running || press_running
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        self.surface.driver_mut()
    }
}

impl Render for SwitchState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.progress.is_running()
            || self.surface.is_animating()
            || self.press_progress.is_running()
        {
            self.schedule_next(window, cx);
        }

        if self.surface.pressed != self.pressed_observed {
            self.pressed_observed = self.surface.pressed;
            let spec = *cx.theme().motion().spec(MotionRole::FastSpatial);
            let target = if self.surface.pressed { 1.0 } else { 0.0 };
            self.press_progress
                .animate_to(target, &spec, Instant::now());
        }

        let theme = cx.theme();
        let colors = theme.colors();
        let disabled = self.disabled;
        let style = SwitchStyle::resolve(theme.token_set(), disabled);
        let state_layer = *theme.state_layer();
        let p = self.progress.value() as f32;
        let press = self.press_progress.value() as f32;

        let track_bg = lerp_color(style.track_off, style.track_on, p);

        let hover_t = if disabled {
            0.0
        } else {
            self.surface.hover_progress() as f32
        };
        let handle_off = lerp_color(style.handle_off, colors.on_surface_variant, hover_t);
        let handle_on = lerp_color(style.handle_on, colors.primary_container, hover_t);
        let handle_bg = lerp_color(handle_off, handle_on, p);

        let (track_w, track_h) = (style.track_size.0, style.track_size.1);
        let rest_size = if self.check_icon {
            style.thumb_on
        } else {
            style.thumb_off + (style.thumb_on - style.thumb_off) * p
        };
        let thumb_size = rest_size + (px(PRESSED_HANDLE_SIZE) - rest_size) * press;
        let center_x = thumb_center(style.track_size, p);
        let thumb_x = center_x - thumb_size / 2.;
        let thumb_y = (px(TOUCH_TARGET_HEIGHT) - thumb_size) / 2.;

        let state_color = lerp_color(colors.on_surface, colors.primary, p);
        let state_opacity = if disabled {
            0.0
        } else {
            hover_t * STATE_LAYER_OPACITY
        };

        let border_alpha = (1.0 - p * 2.0).clamp(0.0, 1.0);
        let border_color = style
            .border_off
            .map(|c| lerp_color(c, Hsla::transparent_black(), 1.0 - border_alpha));

        let (displayed_icon, icon_color) = if p >= 0.5 {
            (
                IconName::new("check"),
                if disabled {
                    colors.disabled_content(&state_layer)
                } else {
                    colors.primary
                },
            )
        } else {
            (
                IconName::new("close"),
                if disabled {
                    colors
                        .surface_container_highest
                        .opacity(state_layer.disabled_content)
                } else {
                    colors.surface_container_highest
                },
            )
        };
        let icon_alpha = if p >= 0.5 {
            2.0 * p - 1.0
        } else {
            1.0 - 2.0 * p
        }
        .clamp(0.0, 1.0);

        let entity = cx.entity();
        let mut root = div()
            .id(self.id.clone())
            .relative()
            .w(track_w)
            .h(px(TOUCH_TARGET_HEIGHT))
            .flex_none()
            .when(!disabled, |el| el.cursor_pointer());

        if !disabled {
            let motion = *theme.motion();
            let hover_entity = entity.clone();
            root = root
                .on_hover(move |hovered, _window, cx| {
                    hover_entity.update(cx, |state, cx| {
                        state.surface.set_hovered(*hovered, &motion, Instant::now());
                        cx.notify();
                    });
                })
                .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
                .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
                .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up));
        }

        if state_opacity > 0.0 {
            root = root.child(
                div()
                    .absolute()
                    .left(center_x - px(STATE_LAYER_SIZE / 2.))
                    .top((px(TOUCH_TARGET_HEIGHT) - px(STATE_LAYER_SIZE)) / 2.)
                    .size(px(STATE_LAYER_SIZE))
                    .rounded_full()
                    .bg(state_color.opacity(state_opacity)),
            );
        }

        let track = div()
            .absolute()
            .left(px(0.))
            .top((px(TOUCH_TARGET_HEIGHT) - track_h) / 2.)
            .w(track_w)
            .h(track_h)
            .rounded_full()
            .bg(track_bg)
            .when_some(border_color, |el, color| {
                el.child(
                    div()
                        .absolute()
                        .inset_0()
                        .rounded_full()
                        .border_2()
                        .border_color(color),
                )
            });
        root = root.child(track);

        if !disabled {
            let ripple_color = lerp_color(colors.on_surface, colors.primary, p);
            self.surface
                .set_ripple_origin(point(center_x, px(TOUCH_TARGET_HEIGHT / 2.)));
            root = self
                .surface
                .overlay_unclipped(ripple_color, state_layer.pressed)
                .apply(root);
            root = root.child(self.surface.bounds.capture_element());
        }

        if !disabled && self.pressed {
            let entity = entity;
            root = root.child(
                canvas(
                    |_, _, _| {},
                    move |_, _, window, _| {
                        let move_entity = entity.clone();
                        window.on_mouse_event(move |event: &MouseMoveEvent, phase, _window, cx| {
                            if phase != DispatchPhase::Bubble {
                                return;
                            }
                            move_entity.update(cx, |state, cx| state.drag_move(event.position, cx));
                        });
                        window.on_mouse_event(move |event: &MouseUpEvent, phase, window, cx| {
                            if phase != DispatchPhase::Bubble || event.button != MouseButton::Left {
                                return;
                            }
                            entity
                                .update(cx, |state, cx| state.drag_end(event.position, window, cx));
                        });
                    },
                )
                .absolute()
                .inset_0(),
            );
        }

        let mut thumb = div()
            .absolute()
            .left(thumb_x)
            .top(thumb_y)
            .size(thumb_size)
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .bg(handle_bg);
        if self.check_icon {
            thumb = thumb.when(icon_alpha > 0.0, |el| {
                el.child(
                    Icon::new(displayed_icon)
                        .size(style.icon_size)
                        .color(icon_color.opacity(icon_alpha)),
                )
            });
        }
        root.child(thumb)
    }
}

pub use appearance::SwitchStyle;

mod appearance {
    use crate::theme::TokenSet;
    use gpui::{Hsla, Pixels, px};

    #[derive(Clone, Copy, Debug)]
    pub struct SwitchStyle {
        pub track_off: Hsla,

        pub track_on: Hsla,

        pub handle_off: Hsla,

        pub handle_on: Hsla,

        pub border_off: Option<Hsla>,

        pub track_size: (Pixels, Pixels),

        pub thumb_on: Pixels,

        pub thumb_off: Pixels,

        pub icon_size: Pixels,

        pub state_layer_color: Hsla,

        pub state_layer_opacity: f32,
    }
    impl SwitchStyle {
        pub fn resolve(tokens: &TokenSet, disabled: bool) -> Self {
            let colors = &tokens.colors;
            let state = &tokens.state_layer;
            let switch = &tokens.component.switch;
            if disabled {
                Self {
                    track_off: colors
                        .surface_container_highest
                        .opacity(state.disabled_container),
                    track_on: colors.disabled_container(state),
                    handle_off: colors.disabled_content(state),

                    handle_on: colors.surface,
                    border_off: Some(colors.disabled_container(state)),
                    track_size: (px(switch.track_width), px(switch.track_height)),
                    thumb_on: px(switch.thumb_size),
                    thumb_off: px(switch.unselected_thumb_size),
                    icon_size: px(switch.icon_size),
                    state_layer_color: colors.on_surface,
                    state_layer_opacity: state.pressed,
                }
            } else {
                Self {
                    track_off: colors.surface_container_highest,
                    track_on: colors.primary,
                    handle_off: colors.outline,
                    handle_on: colors.on_primary,
                    border_off: Some(colors.outline),
                    track_size: (px(switch.track_width), px(switch.track_height)),
                    thumb_on: px(switch.thumb_size),
                    thumb_off: px(switch.unselected_thumb_size),
                    icon_size: px(switch.icon_size),
                    state_layer_color: colors.on_surface,
                    state_layer_opacity: state.pressed,
                }
            }
        }
    }
}
