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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Slider.kt
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/controls/M3Slider.java
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/skins/M3SliderSkin.java

use std::cell::Cell;
use std::rc::Rc;
use std::time::Instant;

use gpui::prelude::FluentBuilder as _;
use gpui::{
    App, AppContext as _, Bounds, Context, DispatchPhase, ElementId, Entity,
    InteractiveElement as _, IntoElement, MouseButton, MouseDownEvent, MouseMoveEvent,
    MouseUpEvent, ParentElement as _, PathBuilder, Pixels, Render, RenderOnce, Styled, Window,
    canvas, div, point, px, relative,
};

use crate::icon::{Icon, IconName};
use crate::motion::{Animatable, AnimatedComponent, AnimationDriver, MotionRole};
use crate::theme::ActiveTheme;

type ChangeHandler = Rc<dyn Fn(f32, &mut Window, &mut App) + 'static>;

const HANDLE_WIDTH: f32 = 4.;
const PRESSED_HANDLE_WIDTH: f32 = 2.;
const TRACK_GAP: f32 = 6.;
const TOUCH_TARGET: f32 = 48.;
const VALUE_INDICATOR_HEIGHT: f32 = 44.;
const VALUE_INDICATOR_GAP: f32 = 12.;
const STOP_INDICATOR_SIZE: f32 = 4.;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SliderSize {
    #[default]
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl SliderSize {
    fn track_thickness(self) -> f32 {
        match self {
            Self::ExtraSmall => 16.,
            Self::Small => 24.,
            Self::Medium => 40.,
            Self::Large => 56.,
            Self::ExtraLarge => 96.,
        }
    }

    fn handle_length(self) -> f32 {
        match self {
            Self::ExtraSmall => 44.,
            Self::Small => 44.,
            Self::Medium => 52.,
            Self::Large => 68.,
            Self::ExtraLarge => 108.,
        }
    }

    fn icon_size(self) -> f32 {
        match self {
            Self::Medium | Self::Large => 24.,
            Self::ExtraLarge => 32.,
            _ => 0.,
        }
    }

    fn icon_padding(self) -> f32 {
        match self {
            Self::Medium | Self::Large => 6.,
            Self::ExtraLarge => 8.,
            _ => 0.,
        }
    }

    fn inset(self) -> f32 {
        HANDLE_WIDTH / 2. + TRACK_GAP
    }
}

fn value_at_position(
    position: f32,
    width: f32,
    inset: f32,
    min: f32,
    max: f32,
    step: Option<f32>,
) -> Option<f32> {
    let travel = width - inset * 2.;
    if !position.is_finite() || !travel.is_finite() || travel <= 0. {
        return None;
    }
    let fraction = ((position - inset) / travel).clamp(0., 1.);
    if fraction <= 0. {
        return Some(min);
    }
    if fraction >= 1. {
        return Some(max);
    }
    let mut value = min + fraction * (max - min);
    if let Some(step) = step.filter(|step| step.is_finite() && *step > 0.) {
        value = ((value - min) / step).round() * step + min;
    }
    Some(value.clamp(min, max))
}

pub struct Slider {
    min: f32,
    max: f32,
    value: f32,
    step: Option<f32>,
    size: SliderSize,
    centered: bool,
    disabled: bool,
    vertical: bool,
    show_value_indicator: bool,
    active_icon: Option<IconName>,
    inactive_icon: Option<IconName>,
    on_change: Option<ChangeHandler>,
}

pub struct SliderState {
    min: f32,
    max: f32,
    value: f32,
    step: Option<f32>,
    size: SliderSize,
    centered: bool,
    disabled: bool,
    vertical: bool,
    show_value_indicator: bool,
    active_icon: Option<IconName>,
    inactive_icon: Option<IconName>,
    dragging: bool,
    pressed: bool,
    fraction: Animatable,
    handle_width: Animatable,
    bounds: Bounds<Pixels>,
    on_change: Option<ChangeHandler>,
    driver: AnimationDriver,
}

impl Slider {
    pub fn new(min: f32, max: f32, value: f32) -> Self {
        Self {
            min,
            max,
            value: value.clamp(min, max),
            step: None,
            size: SliderSize::default(),
            centered: false,
            disabled: false,
            vertical: false,
            show_value_indicator: false,
            active_icon: None,
            inactive_icon: None,
            on_change: None,
        }
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    pub fn size(mut self, size: SliderSize) -> Self {
        self.size = size;
        self
    }

    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn enabled(self, enabled: bool) -> Self {
        self.disabled(!enabled)
    }

    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }

    pub fn show_value_indicator(mut self, show: bool) -> Self {
        self.show_value_indicator = show;
        self
    }

    pub fn active_track_icon(mut self, icon: IconName) -> Self {
        self.active_icon = Some(icon);
        self
    }

    pub fn inactive_track_icon(mut self, icon: IconName) -> Self {
        self.inactive_icon = Some(icon);
        self
    }

    pub fn on_change(mut self, handler: impl Fn(f32, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    pub fn on_value_change(self, handler: impl Fn(f32, &mut Window, &mut App) + 'static) -> Self {
        self.on_change(handler)
    }

    pub fn build(self, cx: &mut App) -> Entity<SliderState> {
        let fraction = if self.max > self.min {
            (self.value - self.min) / (self.max - self.min)
        } else {
            0.
        };
        cx.new(|_| SliderState {
            min: self.min,
            max: self.max,
            value: self.value,
            step: self.step,
            size: self.size,
            centered: self.centered,
            disabled: self.disabled,
            vertical: self.vertical,
            show_value_indicator: self.show_value_indicator,
            active_icon: self.active_icon,
            inactive_icon: self.inactive_icon,
            dragging: false,
            pressed: false,
            fraction: Animatable::new(fraction.clamp(0., 1.) as f64, 1.0e-3),
            handle_width: Animatable::new(HANDLE_WIDTH as f64, 1.0e-3),
            bounds: Bounds::default(),
            on_change: self.on_change,
            driver: AnimationDriver::default(),
        })
    }
}

pub struct VerticalSlider(Slider);

impl VerticalSlider {
    pub fn new(min: f32, max: f32, value: f32) -> Self {
        Self(Slider::new(min, max, value).vertical())
    }

    pub fn step(mut self, step: f32) -> Self {
        self.0 = self.0.step(step);
        self
    }

    pub fn size(mut self, size: SliderSize) -> Self {
        self.0 = self.0.size(size);
        self
    }

    pub fn centered(mut self, centered: bool) -> Self {
        self.0 = self.0.centered(centered);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.0 = self.0.enabled(enabled);
        self
    }

    pub fn on_value_change(
        mut self,
        handler: impl Fn(f32, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.0 = self.0.on_value_change(handler);
        self
    }

    pub fn build(self, cx: &mut App) -> Entity<SliderState> {
        self.0.build(cx)
    }
}

impl SliderState {
    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn set_value(&mut self, value: f32, window: &mut Window, cx: &mut Context<Self>) {
        let mut value = value.clamp(self.min, self.max);
        if let Some(step) = self.step.filter(|step| step.is_finite() && *step > 0.) {
            value = ((value - self.min) / step).round() * step + self.min;
        }
        if (value - self.value).abs() <= f32::EPSILON {
            return;
        }
        self.value = value;
        let target = self.fraction_of(value);
        if self.dragging {
            self.fraction.stop();
            self.fraction.snap_to(target as f64);
        } else {
            self.animate_fraction_to(target, window, cx);
        }
        cx.notify();
    }

    fn fraction_of(&self, value: f32) -> f32 {
        if self.max <= self.min {
            0.
        } else {
            ((value - self.min) / (self.max - self.min)).clamp(0., 1.)
        }
    }

    fn animate_fraction_to(&mut self, target: f32, window: &mut Window, cx: &mut Context<Self>) {
        let spec = *cx.theme().motion().spec(MotionRole::FastSpatial);
        self.fraction
            .animate_to(target as f64, &spec, Instant::now());
        if self.fraction.is_running() {
            self.schedule_next(window, cx);
        }
    }

    fn update_from_position(
        &mut self,
        position: gpui::Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let inset = self.size.inset();
        let (position, length) = if self.vertical {
            (
                f32::from(self.bounds.size.height - (position.y - self.bounds.origin.y)),
                f32::from(self.bounds.size.height),
            )
        } else {
            (
                f32::from(position.x - self.bounds.origin.x),
                f32::from(self.bounds.size.width),
            )
        };
        let Some(value) = value_at_position(position, length, inset, self.min, self.max, self.step)
        else {
            return;
        };
        if (value - self.value).abs() > f32::EPSILON {
            self.value = value;
            self.fraction.stop();
            self.fraction.snap_to(self.fraction_of(value) as f64);
            if let Some(handler) = self.on_change.clone() {
                handler(value, window, cx);
            }
            cx.notify();
        }
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.disabled {
            return;
        }
        self.dragging = true;
        self.pressed = true;
        let spec = *cx.theme().motion().spec(MotionRole::FastEffects);
        self.handle_width
            .animate_to(PRESSED_HANDLE_WIDTH as f64, &spec, Instant::now());
        self.update_from_position(event.position, window, cx);
        if self.handle_width.is_running() {
            self.schedule_next(window, cx);
        }
        cx.notify();
    }

    fn on_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.dragging && event.pressed_button == Some(MouseButton::Left) {
            self.update_from_position(event.position, window, cx);
        }
    }

    fn on_mouse_up(&mut self, event: &MouseUpEvent, window: &mut Window, cx: &mut Context<Self>) {
        if self.dragging {
            self.update_from_position(event.position, window, cx);
            self.dragging = false;
            self.pressed = false;
            let spec = *cx.theme().motion().spec(MotionRole::FastEffects);
            self.handle_width
                .animate_to(HANDLE_WIDTH as f64, &spec, Instant::now());
            if self.handle_width.is_running() {
                self.schedule_next(window, cx);
            }
            cx.notify();
        }
    }
}

impl AnimatedComponent for SliderState {
    fn step(&mut self, now: Instant) -> bool {
        let fraction_moving = self.fraction.tick(now);
        let handle_moving = self.handle_width.tick(now);
        fraction_moving || handle_moving
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }
}

impl Render for SliderState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.fraction.is_running() || self.handle_width.is_running() {
            self.schedule_next(window, cx);
        }

        let disabled = self.disabled;
        let style = SliderStyle::resolve(cx.theme().token_set(), disabled);
        let size = self.size;
        let thickness = px(size.track_thickness());
        let handle_length = px(size.handle_length());
        let handle_width = px(self.handle_width.value() as f32);
        let fraction = self.fraction.value() as f32;
        let active_start = if self.centered {
            0.5_f32.min(fraction)
        } else {
            0.
        };
        let active_end = if self.centered {
            0.5_f32.max(fraction)
        } else {
            fraction
        };
        let show_indicator = self.show_value_indicator && !self.vertical && !disabled;
        let indicator_visible = show_indicator && (self.dragging || self.pressed);
        let entity = cx.entity();

        let icon_size = px(size.icon_size());
        let active_icon = self
            .active_icon
            .clone()
            .filter(|_| f32::from(icon_size) > 0.)
            .map(|icon| {
                Icon::new(icon)
                    .size(icon_size)
                    .color(style.tick_active)
                    .into_any_element()
            });
        let inactive_icon = self
            .inactive_icon
            .clone()
            .filter(|_| f32::from(icon_size) > 0.)
            .map(|icon| {
                Icon::new(icon)
                    .size(icon_size)
                    .color(style.tick_inactive)
                    .into_any_element()
            });

        let vertical = self.vertical;
        let inset_x = px(size.inset());
        let inset_area = || {
            div()
                .absolute()
                .top_0()
                .bottom_0()
                .when(vertical, |el| el.left_0().right_0())
                .when(!vertical, |el| el.left(inset_x).right(inset_x))
        };

        let track = inset_area()
            .child(
                div()
                    .absolute()
                    .when(self.vertical, |el| {
                        el.w(thickness)
                            .left(px((TOUCH_TARGET - f32::from(thickness)) * 0.5))
                            .top_0()
                            .bottom_0()
                    })
                    .when(!self.vertical, |el| {
                        el.top((handle_length - thickness) * 0.5)
                            .h(thickness)
                            .left_0()
                            .right_0()
                    })
                    .rounded_full()
                    .bg(style.inactive_track)
                    .children(inactive_icon.map(|icon| {
                        div()
                            .absolute()
                            .inset_0()
                            .flex()
                            .items_center()
                            .justify_end()
                            .p(px(size.icon_padding()))
                            .child(icon)
                    })),
            )
            .child(
                div()
                    .absolute()
                    .when(self.vertical, |el| {
                        el.w(thickness)
                            .left(px((TOUCH_TARGET - f32::from(thickness)) * 0.5))
                            .top(relative(1. - active_end))
                            .bottom(relative(active_start))
                    })
                    .when(!self.vertical, |el| {
                        el.h(thickness)
                            .top((handle_length - thickness) * 0.5)
                            .left(relative(active_start))
                            .right(relative(1. - active_end))
                    })
                    .rounded_full()
                    .bg(style.active_track)
                    .children(active_icon.map(|icon| {
                        div()
                            .absolute()
                            .inset_0()
                            .flex()
                            .items_center()
                            .justify_start()
                            .p(px(size.icon_padding()))
                            .child(icon)
                    })),
            );

        let ticks = (self
            .step
            .filter(|step| step.is_finite() && *step > 0.)
            .is_some()
            && (self.max - self.min) > 0.)
            .then(|| {
                let vertical = self.vertical;
                let centered = self.centered;
                let disabled = self.disabled;
                let active_low = active_start.min(active_end);
                let active_high = active_start.max(active_end);
                let step_count = (((self.max - self.min) / self.step.unwrap_or(1.)).round() as u32)
                    .clamp(1, 256);
                canvas(
                    move |_, _, _| (),
                    move |bounds, (), window, _| {
                        let travel = if vertical {
                            f32::from(bounds.size.height)
                        } else {
                            f32::from(bounds.size.width)
                        };
                        if travel <= STOP_INDICATOR_SIZE * 2. {
                            return;
                        }
                        let half = px(STOP_INDICATOR_SIZE * 0.5);
                        for index in 0..=step_count {
                            let tick_fraction = index as f32 / step_count as f32;
                            let active_side = if centered {
                                tick_fraction >= active_low && tick_fraction <= active_high
                            } else {
                                tick_fraction <= fraction
                            };
                            let color = if disabled {
                                style.tick_inactive
                            } else if active_side {
                                style.tick_active
                            } else {
                                style.tick_inactive
                            };
                            let center = if vertical {
                                point(
                                    bounds.center().x,
                                    bounds.origin.y + px(tick_fraction * travel),
                                )
                            } else {
                                point(
                                    bounds.origin.x + px(tick_fraction * travel),
                                    bounds.center().y,
                                )
                            };
                            let mut builder = PathBuilder::fill();
                            builder.move_to(point(center.x, center.y - half));
                            builder.line_to(point(center.x + half, center.y));
                            builder.line_to(point(center.x, center.y + half));
                            builder.line_to(point(center.x - half, center.y));
                            builder.close();
                            if let Ok(path) = builder.build() {
                                window.paint_path(path, color);
                            }
                        }
                    },
                )
                .absolute()
                .inset_0()
            });

        let handle = inset_area()
            .child(
                div()
                    .absolute()
                    .when(self.vertical, |el| {
                        el.h(handle_width)
                            .w(handle_length)
                            .left(px((TOUCH_TARGET - f32::from(handle_length)) * 0.5))
                            .top(relative(1. - fraction))
                    })
                    .when(!self.vertical, |el| {
                        el.w(handle_width)
                            .h(handle_length)
                            .left(relative(fraction))
                            .top(px(0.))
                    })
                    .rounded_full()
                    .bg(style.handle),
            )
            .children(ticks);

        let (min, max) = (self.min, self.max);
        let indicator = show_indicator.then(|| {
            let label = format!("{:.0}", min + fraction * (max - min));
            div()
                .absolute()
                .top_0()
                .left_0()
                .right_0()
                .h(px(VALUE_INDICATOR_HEIGHT + VALUE_INDICATOR_GAP))
                .child(
                    div()
                        .absolute()
                        .h(px(VALUE_INDICATOR_HEIGHT))
                        .min_w(px(48.))
                        .left(relative(fraction))
                        .ml(px(-24.))
                        .px(px(12.))
                        .rounded_full()
                        .bg(style.value_indicator)
                        .flex()
                        .items_center()
                        .justify_center()
                        .opacity(if indicator_visible { 1. } else { 0. })
                        .child(
                            cx.theme()
                                .typography()
                                .label_large
                                .apply(div())
                                .text_color(style.value_indicator_text)
                                .child(label),
                        ),
                )
        });

        let interactive = div()
            .relative()
            .flex_1()
            .min_h_0()
            .min_w_0()
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .on_mouse_move(cx.listener(Self::on_mouse_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .child(track)
            .child(handle)
            .child({
                let entity = entity.clone();
                canvas(
                    move |bounds, _window, cx| {
                        entity.update(cx, |this, _| this.bounds = bounds);
                    },
                    |_bounds, _state, _window, _cx| {},
                )
                .absolute()
                .inset_0()
            })
            .when(self.dragging && !disabled, |element| {
                element.child(
                    canvas(
                        |_, _, _| {},
                        move |_, _, window, _| {
                            let move_entity = entity.clone();
                            window.on_mouse_event(
                                move |event: &MouseMoveEvent, phase, window, cx| {
                                    if phase == DispatchPhase::Bubble {
                                        move_entity.update(cx, |state, cx| {
                                            state.on_mouse_move(event, window, cx)
                                        });
                                    }
                                },
                            );
                            window.on_mouse_event(
                                move |event: &MouseUpEvent, phase, window, cx| {
                                    if phase == DispatchPhase::Bubble
                                        && event.button == MouseButton::Left
                                    {
                                        entity.update(cx, |state, cx| {
                                            state.on_mouse_up(event, window, cx)
                                        });
                                    }
                                },
                            );
                        },
                    )
                    .absolute()
                    .inset_0(),
                )
            });

        let root = if self.vertical {
            div()
                .id("md3-slider")
                .relative()
                .w(px(TOUCH_TARGET))
                .h(px(200.))
                .min_w_0()
                .flex_none()
        } else {
            let reserve = if show_indicator {
                VALUE_INDICATOR_HEIGHT + VALUE_INDICATOR_GAP
            } else {
                0.
            };
            div()
                .id("md3-slider")
                .relative()
                .w_full()
                .min_w_0()
                .flex_none()
                .h(px(f32::from(handle_length).max(TOUCH_TARGET) + reserve))
                .flex()
                .flex_col()
        };

        if self.vertical {
            root.when(!disabled, |el| el.cursor_pointer())
                .child(div().absolute().inset_0().child(interactive))
        } else {
            root.when(!disabled, |el| el.cursor_pointer())
                .children(indicator)
                .child(interactive)
        }
    }
}

pub use appearance::SliderStyle;

mod appearance {
    use crate::theme::TokenSet;
    use gpui::Hsla;

    #[derive(Clone, Copy, Debug)]
    pub struct SliderStyle {
        pub active_track: Hsla,

        pub inactive_track: Hsla,

        pub handle: Hsla,

        pub tick_active: Hsla,

        pub tick_inactive: Hsla,

        pub value_indicator: Hsla,

        pub value_indicator_text: Hsla,
    }
    impl SliderStyle {
        pub fn resolve(tokens: &TokenSet, disabled: bool) -> Self {
            let colors = &tokens.colors;
            let state = &tokens.state_layer;
            Self {
                active_track: if disabled {
                    colors.disabled_content(state)
                } else {
                    colors.primary
                },
                inactive_track: if disabled {
                    colors.disabled_container(state)
                } else {
                    colors.secondary_container
                },
                handle: if disabled {
                    colors.disabled_content(state)
                } else {
                    colors.primary
                },
                tick_active: if disabled {
                    colors.disabled_content(state)
                } else {
                    colors.on_primary
                },
                tick_inactive: if disabled {
                    colors.disabled_content(state)
                } else {
                    colors.on_secondary_container
                },
                value_indicator: colors.inverse_surface,
                value_indicator_text: colors.inverse_on_surface,
            }
        }
    }
}

type RangeChangeHandler = Rc<dyn Fn((f32, f32), &mut Window, &mut App)>;

#[derive(IntoElement)]
pub struct RangeSlider {
    id: ElementId,
    start: f32,
    end: f32,
    size: SliderSize,
    enabled: bool,
    on_value_change: Option<RangeChangeHandler>,
}

impl RangeSlider {
    pub fn new(id: impl Into<ElementId>, start: f32, end: f32) -> Self {
        Self {
            id: id.into(),
            start: start.clamp(0., 1.).min(end.clamp(0., 1.)),
            end: end.clamp(0., 1.).max(start.clamp(0., 1.)),
            size: SliderSize::default(),
            enabled: true,
            on_value_change: None,
        }
    }

    pub fn range(mut self, start: f32, end: f32) -> Self {
        self.start = start.clamp(0., 1.).min(end.clamp(0., 1.));
        self.end = end.clamp(0., 1.).max(self.start);
        self
    }

    pub fn size(mut self, size: SliderSize) -> Self {
        self.size = size;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn on_value_change(
        mut self,
        handler: impl Fn((f32, f32), &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for RangeSlider {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let style = SliderStyle::resolve(cx.theme().token_set(), !self.enabled);
        let size = self.size;
        let thickness = px(size.track_thickness());
        let handle_length = px(size.handle_length());
        let bounds = crate::interaction::BoundsHandle::new();
        let values = Rc::new(Cell::new((self.start, self.end)));
        let active_start = Rc::new(Cell::new(None::<bool>));
        let on_mouse_down = {
            let bounds = bounds.clone();
            let values = values.clone();
            let active_start = active_start.clone();
            let callback = self.on_value_change.clone();
            move |event: &gpui::MouseDownEvent, window: &mut Window, cx: &mut App| {
                let rect = bounds.get();
                let width = f32::from(rect.size.width);
                if width <= 0. {
                    return;
                }
                let inset = size.inset();
                let travel = width - inset * 2.;
                if travel <= 0. {
                    return;
                }
                let fraction =
                    ((f32::from(event.position.x - rect.origin.x) - inset) / travel).clamp(0., 1.);
                let (start, end) = values.get();
                let is_start = (fraction - start).abs() <= (fraction - end).abs();
                active_start.set(Some(is_start));
                let next = if is_start {
                    (fraction.min(end), end)
                } else {
                    (start, fraction.max(start))
                };
                values.set(next);
                if let Some(handler) = &callback {
                    handler(next, window, cx);
                }
            }
        };
        let on_mouse_move = {
            let bounds = bounds.clone();
            let active_start = active_start.clone();
            let callback = self.on_value_change;
            move |event: &gpui::MouseMoveEvent, window: &mut Window, cx: &mut App| {
                let Some(is_start) = active_start.get() else {
                    return;
                };
                if event.pressed_button != Some(MouseButton::Left) {
                    return;
                }
                let rect = bounds.get();
                let width = f32::from(rect.size.width);
                if width <= 0. {
                    return;
                }
                let inset = size.inset();
                let travel = width - inset * 2.;
                if travel <= 0. {
                    return;
                }
                let fraction =
                    ((f32::from(event.position.x - rect.origin.x) - inset) / travel).clamp(0., 1.);
                let (start, end) = values.get();
                let next = if is_start {
                    (fraction.min(end), end)
                } else {
                    (start, fraction.max(start))
                };
                if next != (start, end) {
                    values.set(next);
                    if let Some(handler) = &callback {
                        handler(next, window, cx);
                    }
                }
            }
        };
        let handle_color = style.handle;
        div()
            .id(self.id)
            .relative()
            .h(px(size.handle_length().max(TOUCH_TARGET)))
            .w_full()
            .min_w_0()
            .when(self.enabled, |el| {
                el.cursor_pointer()
                    .on_mouse_down(MouseButton::Left, on_mouse_down)
                    .on_mouse_move(on_mouse_move)
                    .on_mouse_up(MouseButton::Left, move |_, _, _| active_start.set(None))
            })
            .child(
                div()
                    .absolute()
                    .top_0()
                    .bottom_0()
                    .left(px(size.inset()))
                    .right(px(size.inset()))
                    .child(
                        div()
                            .absolute()
                            .top((handle_length - thickness) * 0.5)
                            .h(thickness)
                            .left_0()
                            .right_0()
                            .rounded_full()
                            .bg(style.inactive_track),
                    )
                    .child(
                        div()
                            .absolute()
                            .top((handle_length - thickness) * 0.5)
                            .h(thickness)
                            .left(relative(self.start))
                            .right(relative(1. - self.end))
                            .rounded_full()
                            .bg(style.active_track),
                    )
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .h(handle_length)
                            .w(px(HANDLE_WIDTH))
                            .left(relative(self.start))
                            .ml(px(-HANDLE_WIDTH / 2.))
                            .rounded_full()
                            .bg(handle_color),
                    )
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .h(handle_length)
                            .w(px(HANDLE_WIDTH))
                            .left(relative(self.end))
                            .ml(px(-HANDLE_WIDTH / 2.))
                            .rounded_full()
                            .bg(handle_color),
                    ),
            )
            .child(bounds.capture_element())
    }
}
