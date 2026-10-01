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

use std::cell::Cell;
use std::rc::Rc;

use gpui::prelude::FluentBuilder as _;
use gpui::{
    App, AppContext as _, Bounds, Context, DispatchPhase, ElementId, Entity,
    InteractiveElement as _, IntoElement, MouseButton, MouseDownEvent, MouseMoveEvent,
    MouseUpEvent, ParentElement as _, Pixels, Render, RenderOnce, Styled, Window, canvas, div, px,
    relative,
};

use crate::theme::ActiveTheme;

type ChangeHandler = Rc<dyn Fn(f32, &mut Window, &mut App) + 'static>;

const TRACK_GAP: f32 = 6.;

fn value_at_position(
    position: f32,
    width: f32,
    handle_width: f32,
    min: f32,
    max: f32,
    step: Option<f32>,
) -> Option<f32> {
    let inset = handle_width / 2. + TRACK_GAP;
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
    disabled: bool,
    vertical: bool,
    on_change: Option<ChangeHandler>,
}

pub struct SliderState {
    min: f32,
    max: f32,
    value: f32,
    step: Option<f32>,
    disabled: bool,
    vertical: bool,
    dragging: bool,
    bounds: Bounds<Pixels>,
    on_change: Option<ChangeHandler>,
}

impl Slider {
    pub fn new(min: f32, max: f32, value: f32) -> Self {
        Self {
            min,
            max,
            value: value.clamp(min, max),
            step: None,
            disabled: false,
            vertical: false,
            on_change: None,
        }
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
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

    pub fn on_change(mut self, handler: impl Fn(f32, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    pub fn on_value_change(self, handler: impl Fn(f32, &mut Window, &mut App) + 'static) -> Self {
        self.on_change(handler)
    }

    pub fn build(self, cx: &mut App) -> Entity<SliderState> {
        cx.new(|_| SliderState {
            min: self.min,
            max: self.max,
            value: self.value,
            step: self.step,
            disabled: self.disabled,
            vertical: self.vertical,
            dragging: false,
            bounds: Bounds::default(),
            on_change: self.on_change,
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

    pub fn set_value(&mut self, value: f32, cx: &mut Context<Self>) {
        self.value = value.clamp(self.min, self.max);
        cx.notify();
    }

    fn fraction(&self) -> f32 {
        if self.max <= self.min {
            0.
        } else {
            (self.value - self.min) / (self.max - self.min)
        }
    }

    fn update_from_position(
        &mut self,
        position: gpui::Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
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
        let Some(value) = value_at_position(
            position,
            length,
            cx.theme().component().slider.handle_width,
            self.min,
            self.max,
            self.step,
        ) else {
            return;
        };
        if (value - self.value).abs() > f32::EPSILON {
            self.value = value;
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
        self.update_from_position(event.position, window, cx);
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
            cx.notify();
        }
    }
}

impl Render for SliderState {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let disabled = self.disabled;
        let style = SliderStyle::resolve(cx.theme().token_set(), disabled);
        let fraction = self.fraction().clamp(0., 1.);

        let active_color = style.active_track;
        let inactive_color = style.inactive_track;
        let handle_color = style.handle;

        let entity = cx.entity();
        let track_content = if self.vertical {
            div()
                .size_full()
                .flex()
                .flex_col()
                .items_center()
                .gap(px(TRACK_GAP))
                .child(
                    div()
                        .w(style.track_height)
                        .min_h_0()
                        .flex_basis(px(0.))
                        .map(|mut track| {
                            track.style().flex_grow = Some(1. - fraction);
                            track
                        })
                        .rounded_full()
                        .bg(inactive_color),
                )
                .child(
                    div()
                        .w(style.handle_size.1)
                        .h(style.handle_size.0)
                        .flex_none()
                        .rounded_full()
                        .bg(handle_color),
                )
                .child(
                    div()
                        .w(style.track_height)
                        .min_h_0()
                        .flex_basis(px(0.))
                        .map(|mut track| {
                            track.style().flex_grow = Some(fraction);
                            track
                        })
                        .rounded_full()
                        .bg(active_color),
                )
                .into_any_element()
        } else {
            div()
                .size_full()
                .flex()
                .items_center()
                .gap(px(TRACK_GAP))
                .child(
                    div()
                        .h(style.track_height)
                        .min_w_0()
                        .flex_basis(px(0.))
                        .map(|mut track| {
                            track.style().flex_grow = Some(fraction);
                            track
                        })
                        .rounded_tl(style.track_height / 2.)
                        .rounded_bl(style.track_height / 2.)
                        .rounded_tr(px(2.))
                        .rounded_br(px(2.))
                        .bg(active_color),
                )
                .child(
                    div()
                        .w(style.handle_size.0)
                        .h(style.handle_size.1)
                        .flex_none()
                        .rounded_full()
                        .bg(handle_color),
                )
                .child(
                    div()
                        .h(style.track_height)
                        .min_w_0()
                        .flex_basis(px(0.))
                        .map(|mut track| {
                            track.style().flex_grow = Some(1. - fraction);
                            track
                        })
                        .rounded_tl(px(2.))
                        .rounded_bl(px(2.))
                        .rounded_tr(style.track_height / 2.)
                        .rounded_br(style.track_height / 2.)
                        .bg(inactive_color),
                )
                .into_any_element()
        };

        div()
            .id("md3-slider")
            .relative()
            .when(self.vertical, |el| el.w(style.container_height).h(px(200.)))
            .when(!self.vertical, |el| {
                el.w_full().min_w_0().h(style.container_height)
            })
            .when(!disabled, |el| el.cursor_pointer())
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .on_mouse_move(cx.listener(Self::on_mouse_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .child(track_content)
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
            })
    }
}

pub use appearance::SliderStyle;

mod appearance {
    use crate::theme::TokenSet;
    use gpui::{Hsla, Pixels, px};

    #[derive(Clone, Copy, Debug)]
    pub struct SliderStyle {
        pub active_track: Hsla,

        pub inactive_track: Hsla,

        pub handle: Hsla,

        pub track_height: Pixels,

        pub handle_size: (Pixels, Pixels),

        pub container_height: Pixels,
    }
    impl SliderStyle {
        pub fn resolve(tokens: &TokenSet, disabled: bool) -> Self {
            let colors = &tokens.colors;
            let state = &tokens.state_layer;
            let slider = &tokens.component.slider;
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
                track_height: px(slider.track_height),
                handle_size: (px(slider.handle_width), px(slider.handle_height)),
                container_height: px(44.),
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
    enabled: bool,
    on_value_change: Option<RangeChangeHandler>,
}
impl RangeSlider {
    pub fn new(id: impl Into<ElementId>, start: f32, end: f32) -> Self {
        Self {
            id: id.into(),
            start: start.clamp(0., 1.).min(end.clamp(0., 1.)),
            end: end.clamp(0., 1.).max(start.clamp(0., 1.)),
            enabled: true,
            on_value_change: None,
        }
    }
    pub fn range(mut self, start: f32, end: f32) -> Self {
        self.start = start.clamp(0., 1.).min(end.clamp(0., 1.));
        self.end = end.clamp(0., 1.).max(self.start);
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
        let c = cx.theme().colors();
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
                let fraction = (f32::from(event.position.x - rect.origin.x) / width).clamp(0., 1.);
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
                let fraction = (f32::from(event.position.x - rect.origin.x) / width).clamp(0., 1.);
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
        div()
            .id(self.id)
            .relative()
            .h(px(44.))
            .w_full()
            .when(self.enabled, |el| {
                el.cursor_pointer()
                    .on_mouse_down(MouseButton::Left, on_mouse_down)
                    .on_mouse_move(on_mouse_move)
                    .on_mouse_up(MouseButton::Left, move |_, _, _| active_start.set(None))
            })
            .child(
                div()
                    .absolute()
                    .top(px(20.))
                    .h(px(4.))
                    .w_full()
                    .rounded_full()
                    .bg(c.secondary_container),
            )
            .child(
                div()
                    .absolute()
                    .top(px(20.))
                    .left(relative(self.start))
                    .right(relative(1. - self.end))
                    .h(px(4.))
                    .bg(c.primary),
            )
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left(relative(self.start))
                    .ml(px(-2.))
                    .w(px(4.))
                    .h(px(44.))
                    .rounded_full()
                    .bg(c.primary),
            )
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left(relative(self.end))
                    .ml(px(-2.))
                    .w(px(4.))
                    .h(px(44.))
                    .rounded_full()
                    .bg(c.primary),
            )
            .child(bounds.capture_element())
    }
}
