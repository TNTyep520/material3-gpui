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
    App, AppContext as _, Bounds, Context, Entity, InteractiveElement as _, IntoElement,
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, ParentElement as _, Pixels, Render,
    Styled, Window, canvas, div, px, relative,
};

use crate::theme::{ActiveTheme, DISABLED_CONTENT_OPACITY};

type ChangeHandler = Rc<dyn Fn(f32, &mut Window, &mut App) + 'static>;

/// MD3 滑块构建器（`.build(cx)` 产出 [`SliderState`]）。
pub struct Slider {
    min: f32,
    max: f32,
    value: f32,
    step: Option<f32>,
    disabled: bool,
    on_change: Option<ChangeHandler>,
}

/// 滑块的有状态部分：值与拖拽状态。
pub struct SliderState {
    min: f32,
    max: f32,
    value: f32,
    step: Option<f32>,
    disabled: bool,
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

    /// 值变化回调。
    pub fn on_change(mut self, handler: impl Fn(f32, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// 构建有状态组件实体。
    pub fn build(self, cx: &mut App) -> Entity<SliderState> {
        cx.new(|_| SliderState {
            min: self.min,
            max: self.max,
            value: self.value,
            step: self.step,
            disabled: self.disabled,
            dragging: false,
            bounds: Bounds::default(),
            on_change: self.on_change,
        })
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

    fn update_from_x(&mut self, x: Pixels, window: &mut Window, cx: &mut Context<Self>) {
        let width = f32::from(self.bounds.size.width);
        if width <= 0. {
            return;
        }
        let rel = (f32::from(x) - f32::from(self.bounds.origin.x)) / width;
        let mut value = self.min + rel.clamp(0., 1.) * (self.max - self.min);
        if let Some(step) = self.step
            && step > 0.
        {
            value = ((value - self.min) / step).round() * step + self.min;
        }
        let value = value.clamp(self.min, self.max);
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
        self.update_from_x(event.position.x, window, cx);
    }

    fn on_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.dragging && event.pressed_button == Some(MouseButton::Left) {
            self.update_from_x(event.position.x, window, cx);
        }
    }

    fn on_mouse_up(&mut self, _event: &MouseUpEvent, _window: &mut Window, cx: &mut Context<Self>) {
        if self.dragging {
            self.dragging = false;
            cx.notify();
        }
    }
}

impl Render for SliderState {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors();
        let state_layer = *cx.theme().state_layer();
        let tokens = cx.theme().component().slider;
        let disabled = self.disabled;
        let fraction = self.fraction().clamp(0., 1.);

        let active_color = if disabled {
            colors.on_surface.opacity(DISABLED_CONTENT_OPACITY)
        } else {
            colors.primary
        };
        let inactive_color = if disabled {
            colors.disabled_container(&state_layer)
        } else {
            colors.secondary_container
        };
        let handle_color = active_color;

        let entity = cx.entity();

        div()
            .id("md3-slider")
            .relative()
            .w_full()
            .h(px(44.))
            .flex()
            .items_center()
            .gap(px(6.))
            .when(!disabled, |el| el.cursor_pointer())
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .on_mouse_move(cx.listener(Self::on_mouse_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up))
            // 活动轨道（允许收缩，避免手柄+间距造成溢出）
            .child(
                div()
                    .h(px(tokens.track_height))
                    .w(relative(fraction))
                    .rounded_tl(px(tokens.track_height / 2.))
                    .rounded_bl(px(tokens.track_height / 2.))
                    .rounded_tr(px(2.))
                    .rounded_br(px(2.))
                    .bg(active_color),
            )
            // 手柄（4×44 竖条）
            .child(
                div()
                    .w(px(tokens.handle_width))
                    .h(px(tokens.handle_height))
                    .flex_none()
                    .rounded_full()
                    .bg(handle_color),
            )
            // 非活动轨道
            .child(
                div()
                    .h(px(tokens.track_height))
                    .flex_1()
                    .rounded_tl(px(2.))
                    .rounded_bl(px(2.))
                    .rounded_tr(px(tokens.track_height / 2.))
                    .rounded_br(px(tokens.track_height / 2.))
                    .bg(inactive_color),
            )
            // 捕获轨道 bounds，用于把鼠标 x 坐标映射为数值
            .child({
                let entity = entity;
                canvas(
                    move |bounds, _window, cx| {
                        entity.update(cx, |this, _| this.bounds = bounds);
                    },
                    |_bounds, _state, _window, _cx| {},
                )
                .absolute()
                .size_full()
            })
    }
}
