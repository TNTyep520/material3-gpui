//! MD3 Slider（对应 material-web / M3 2024 样式的 `md-slider`）。
//!
//! 采用 M3 新版视觉：16dp 高的圆角轨道 + 4×44dp 竖条手柄。
//! 有状态组件，构建器 + `.build(cx)` 产出 [`SliderState`]：
//!
//! ```ignore
//! // 创建（在视图 render 中，cx 为 &mut Context<V>）：
//! Slider::new(0.0, 100.0, 40.0)
//!     .step(10.0)
//!     .on_change(|value, _, _| println!("{value}"))
//!     .build(cx)   // -> Entity<SliderState>
//! ```

use std::rc::Rc;

use gpui::prelude::FluentBuilder as _;
use gpui::{
    App, AppContext as _, Bounds, Context, DispatchPhase, Entity, InteractiveElement as _,
    IntoElement, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, ParentElement as _,
    Pixels, Render, Styled, Window, canvas, div, px,
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

/// MD3 滑块构建器（`.build(cx)` 产出 [`SliderState`]）。
pub struct Slider {
    min: f32,
    max: f32,
    value: f32,
    step: Option<f32>,
    disabled: bool,
    vertical: bool,
    on_change: Option<ChangeHandler>,
}

/// 滑块的有状态部分：值与拖拽状态。
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
    /// 创建滑块构建器。
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

    /// 离散步长（如 10.0）。
    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    /// 设置禁用态。
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置 AndroidX 对应的 enabled 状态。
    pub fn enabled(self, enabled: bool) -> Self {
        self.disabled(!enabled)
    }

    /// 沿纵向布局轨道，顶部表示最大值。
    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }

    /// 值变化回调。
    pub fn on_change(mut self, handler: impl Fn(f32, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// 设置数值变化回调；传递当前值域内的新值。
    pub fn on_value_change(self, handler: impl Fn(f32, &mut Window, &mut App) + 'static) -> Self {
        self.on_change(handler)
    }

    /// 构建有状态组件实体。
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

/// AndroidX VerticalSlider 对应的纵向滑块。
pub struct VerticalSlider(Slider);

impl VerticalSlider {
    /// 创建顶部为最大值的纵向滑块。
    pub fn new(min: f32, max: f32, value: f32) -> Self {
        Self(Slider::new(min, max, value).vertical())
    }

    /// 设置离散步长。
    pub fn step(mut self, step: f32) -> Self {
        self.0 = self.0.step(step);
        self
    }

    /// 设置启用状态。
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.0 = self.0.enabled(enabled);
        self
    }

    /// 设置数值变化回调。
    pub fn on_value_change(
        mut self,
        handler: impl Fn(f32, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.0 = self.0.on_value_change(handler);
        self
    }

    /// 构建可渲染的纵向滑块实体。
    pub fn build(self, cx: &mut App) -> Entity<SliderState> {
        self.0.build(cx)
    }
}

impl SliderState {
    /// 当前值。
    pub fn value(&self) -> f32 {
        self.value
    }

    /// 编程式设置值。
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
            // 捕获轨道 bounds，用于把鼠标 x 坐标映射为数值
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
    /// MD3 Slider 样式。
    #[derive(Clone, Copy, Debug)]
    pub struct SliderStyle {
        /// 活动轨道色。
        pub active_track: Hsla,
        /// 非活动轨道色。
        pub inactive_track: Hsla,
        /// 手柄色。
        pub handle: Hsla,
        /// 轨道高度。
        pub track_height: Pixels,
        /// 手柄宽/高。
        pub handle_size: (Pixels, Pixels),
        /// 轨道容器高。
        pub container_height: Pixels,
    }
    impl SliderStyle {
        /// 由令牌推导默认样式。
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
