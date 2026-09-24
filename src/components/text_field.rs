//! MD3 TextField / TextInputLayout（对应 material-web 的 `md-outlined-text-field`）。
//!
//! 首期提供 Outlined 单行文本框：浮动标签、helper/error 文本、
//! 聚焦指示条加粗（fastEffects 弹簧）、前后缀图标。
//!
//! 文本录入基于按键事件（`on_key_down`），支持字符输入、退格/删除、
//! 方向键移动光标、Home/End、Enter 提交；IME 组合输入暂不支持。
//!
//! ```ignore
//! TextField::new("name", "Name")
//!     .helper("Your display name")
//!     .on_submit(|value, _, _| println!("{value}"))
//!     .build(cx)   // -> Entity<TextFieldState>
//! ```

use std::rc::Rc;
use std::time::Instant;

use gpui::{
    App, AppContext as _, Context, ElementId, Entity, FocusHandle, Focusable,
    InteractiveElement as _, IntoElement, ParentElement as _, Pixels, Render, SharedString, Styled,
    Window, div, prelude::FluentBuilder as _, px,
};

use crate::icon::{Icon, IconName};
use crate::motion::{Animatable, AnimatedComponent, AnimationDriver, MotionRole, lerp_color};
use crate::theme::ActiveTheme;

type ChangeHandler = Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>;
type SubmitHandler = Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>;

/// MD3 单行文本框构建器（`.build(cx)` 产出 [`TextFieldState`]）。
pub struct TextField {
    id: ElementId,
    label: SharedString,
    value: SharedString,
    helper: Option<SharedString>,
    error: Option<SharedString>,
    leading_icon: Option<IconName>,
    disabled: bool,
    on_change: Option<ChangeHandler>,
    on_submit: Option<SubmitHandler>,
}

/// 文本框的有状态部分：值、光标、焦点与动画。
pub struct TextFieldState {
    id: ElementId,
    label: SharedString,
    helper: Option<SharedString>,
    error: Option<SharedString>,
    leading_icon: Option<IconName>,
    disabled: bool,
    value: String,
    /// 光标位置（UTF-8 字符下标）。
    caret: usize,
    focus: FocusHandle,
    /// 0 = 未聚焦，1 = 聚焦（标签浮动/指示条加粗）。
    focus_progress: Animatable,
    on_change: Option<ChangeHandler>,
    on_submit: Option<SubmitHandler>,
    driver: AnimationDriver,
}

impl TextField {
    /// 创建文本框构建器。
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            value: SharedString::default(),
            helper: None,
            error: None,
            leading_icon: None,
            disabled: false,
            on_change: None,
            on_submit: None,
        }
    }

    /// 初始文本。
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }

    /// 辅助文本（显示在下方；有 error 时被 error 替代）。
    pub fn helper(mut self, helper: impl Into<SharedString>) -> Self {
        self.helper = Some(helper.into());
        self
    }

    /// 错误文本（显示在下方并切换 error 配色）。
    pub fn error(mut self, error: impl Into<SharedString>) -> Self {
        self.error = Some(error.into());
        self
    }

    /// 前导图标。
    pub fn leading_icon(mut self, icon: IconName) -> Self {
        self.leading_icon = Some(icon);
        self
    }

    /// 设置禁用态。
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 文本变化回调。
    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// Enter 提交回调。
    pub fn on_submit(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_submit = Some(Rc::new(handler));
        self
    }

    /// 构建有状态组件实体。
    pub fn build(self, cx: &mut App) -> Entity<TextFieldState> {
        let focus = cx.focus_handle();
        let value = self.value.to_string();
        let caret = value.chars().count();
        cx.new(|_| TextFieldState {
            id: self.id,
            label: self.label,
            helper: self.helper,
            error: self.error,
            leading_icon: self.leading_icon,
            disabled: self.disabled,
            value,
            caret,
            focus,
            focus_progress: Animatable::new(0.0, 1.0e-3),
            on_change: self.on_change,
            on_submit: self.on_submit,
            driver: AnimationDriver::default(),
        })
    }
}

impl TextFieldState {
    /// 当前文本。
    pub fn value(&self) -> &str {
        &self.value
    }

    /// 设置文本（光标移到末尾）。
    pub fn set_value(&mut self, value: &str, cx: &mut Context<Self>) {
        self.value = value.to_string();
        self.caret = self.value.chars().count();
        cx.notify();
    }

    /// 是否聚焦（读取焦点句柄）。
    pub fn is_focused(&self, window: &Window) -> bool {
        self.focus.is_focused(window)
    }

    fn sync_focus(&mut self, focused: bool, window: &mut Window, cx: &mut Context<Self>) {
        let target = if focused { 1.0 } else { 0.0 };
        if (self.focus_progress.target() - target).abs() > f64::EPSILON {
            let spec = *cx.theme().motion().spec(MotionRole::FastEffects);
            self.focus_progress
                .animate_to(target, &spec, Instant::now());
            if self.focus_progress.is_running() {
                self.schedule_next(window, cx);
            }
        }
    }

    fn insert_char(&mut self, ch: char, window: &mut Window, cx: &mut Context<Self>) {
        let caret_char = self.caret.min(self.value.chars().count());
        let byte_idx = self
            .value
            .char_indices()
            .nth(caret_char)
            .map(|(i, _)| i)
            .unwrap_or(self.value.len());
        self.value.insert(byte_idx, ch);
        self.caret = caret_char + 1;
        if let Some(handler) = self.on_change.clone() {
            handler(&self.value, window, cx);
        }
        cx.notify();
    }

    fn backspace(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let count = self.value.chars().count();
        if self.caret == 0 || count == 0 {
            return;
        }
        let start = self.caret - 1;
        let byte_start = self.value.char_indices().nth(start).map(|(i, _)| i);
        let byte_end = self.value.char_indices().nth(self.caret).map(|(i, _)| i);
        let byte_start = byte_start.unwrap_or(0);
        let byte_end = byte_end.unwrap_or(self.value.len());
        self.value.replace_range(byte_start..byte_end, "");
        self.caret = start;
        if let Some(handler) = self.on_change.clone() {
            handler(&self.value, window, cx);
        }
        cx.notify();
    }

    fn delete(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let count = self.value.chars().count();
        if self.caret >= count {
            return;
        }
        let byte_start = self.value.char_indices().nth(self.caret).map(|(i, _)| i);
        let byte_end = self
            .value
            .char_indices()
            .nth(self.caret + 1)
            .map(|(i, _)| i);
        let byte_start = byte_start.unwrap_or(0);
        let byte_end = byte_end.unwrap_or(self.value.len());
        self.value.replace_range(byte_start..byte_end, "");
        if let Some(handler) = self.on_change.clone() {
            handler(&self.value, window, cx);
        }
        cx.notify();
    }
}

impl Focusable for TextFieldState {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus.clone()
    }
}

impl AnimatedComponent for TextFieldState {
    fn step(&mut self, now: Instant) -> bool {
        self.focus_progress.tick(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }
}

impl Render for TextFieldState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.focus_progress.is_running() {
            self.schedule_next(window, cx);
        }

        let focused = !self.disabled && self.focus.is_focused(window);
        self.sync_focus(focused, window, cx);
        let caret_x = if focused {
            f32::from(self.caret_x(window, cx))
        } else {
            0.0
        };

        let theme = cx.theme();
        let colors = theme.colors();
        let tokens = theme.component().text_field;
        let state_layer = *theme.state_layer();
        let p = self.focus_progress.value() as f32;

        let has_error = self.error.is_some();
        let accent = if has_error {
            colors.error
        } else {
            colors.primary
        };

        // 边框：未聚焦 1dp outline；聚焦时叠加 2dp accent 描边（透明度随进度）
        let border_color = if self.disabled {
            colors.disabled_container(&state_layer)
        } else if has_error {
            lerp_color(colors.outline, colors.error, p)
        } else {
            lerp_color(colors.outline, accent, p)
        };

        // 浮动标签：聚焦或非空时缩小上浮
        let floating = p > 0.5 || !self.value.is_empty();
        let label_color = if focused && !has_error {
            accent
        } else {
            colors.on_surface_variant
        };
        let label_style = theme.typography();
        let gap = px(tokens.supporting_gap);

        let icon_size = px(tokens.icon_size);
        let min_h = px(tokens.min_height);

        let entity = cx.entity();
        let key_entity = entity;

        let container = div()
            .id(self.id.clone())
            .min_h(min_h)
            .w_full()
            .relative()
            .flex()
            .flex_col()
            .justify_center()
            .rounded(theme.shapes().extra_small)
            .border_1()
            .border_color(border_color)
            .bg(colors.surface)
            .when(self.disabled, |el| {
                el.bg(colors
                    .on_surface
                    .opacity(state_layer.disabled_container / 6.0))
            })
            .when(!self.disabled, |el| el.cursor_text())
            // 点击聚焦
            .on_mouse_down(gpui::MouseButton::Left, {
                let focus = self.focus.clone();
                move |_event, window, cx| window.focus(&focus, cx)
            })
            // 键盘录入
            .on_key_down(move |event, window, cx| {
                key_entity.update(cx, |state, cx| {
                    let handled = state.handle_key(&event.keystroke, window, cx);
                    if handled {
                        cx.stop_propagation();
                    }
                });
            })
            .track_focus(&self.focus)
            // 聚焦指示条：叠加的 2dp accent 描边，透明度随弹簧进度
            .when((focused || p > 0.0) && !self.disabled, |el| {
                el.child(
                    div()
                        .absolute()
                        .inset_0()
                        .rounded(theme.shapes().extra_small)
                        .border_2()
                        .border_color(if has_error {
                            colors.error.opacity(p.max(0.001))
                        } else {
                            accent.opacity(p.max(0.001))
                        }),
                )
            });

        let row = div()
            .flex()
            .items_center()
            .gap(px(tokens.horizontal_padding * 0.5))
            .px(px(tokens.horizontal_padding))
            .py(px(tokens.top_padding));
        let row = row
            .when_some(self.leading_icon, |el, icon| {
                el.child(
                    Icon::new(icon)
                        .size(icon_size)
                        .color(colors.on_surface_variant),
                )
            })
            .child(
                // 文本区（占位 + 文本 + 光标）
                div()
                    .relative()
                    .flex_1()
                    .min_h(px(tokens.min_height
                        - tokens.top_padding
                        - tokens.bottom_padding))
                    .flex()
                    .items_center()
                    .child(
                        div()
                            .text_size(label_style.body_large.size)
                            .line_height(label_style.body_large.line_height)
                            .text_color(colors.on_surface)
                            .child(SharedString::from(self.value.clone())),
                    )
                    .when(self.value.is_empty() && !floating, |el| {
                        el.child(
                            div()
                                .absolute()
                                .left_0()
                                .text_size(label_style.body_large.size)
                                .text_color(colors.on_surface_variant.opacity(0.7))
                                .child(self.label.clone()),
                        )
                    })
                    // 光标
                    .when(focused, |el| {
                        el.child(
                            div()
                                .absolute()
                                .left(px(caret_x))
                                .top(px(6.))
                                .bottom(px(6.))
                                .w(px(2.))
                                .rounded_full()
                                .bg(accent),
                        )
                    }),
            );

        // 浮动标签（容器顶部）
        let label_el = if floating {
            div()
                .absolute()
                .top(px(-8.))
                .left(px(tokens.horizontal_padding))
                .px(px(4.))
                .bg(colors.surface)
                .text_size(label_style.label_small.size)
                .text_color(label_color)
                .child(self.label.clone())
        } else {
            div()
        };

        // helper / error 文本
        let supporting = if let Some(err) = &self.error {
            div()
                .text_size(label_style.body_small.size)
                .text_color(colors.error)
                .child(err.clone())
        } else if let Some(helper) = &self.helper {
            div()
                .text_size(label_style.body_small.size)
                .text_color(colors.on_surface_variant)
                .child(helper.clone())
        } else {
            div()
        };

        container
            .child(
                div()
                    .relative()
                    .flex()
                    .flex_col()
                    .child(label_el)
                    .child(row),
            )
            .when(self.error.is_some() || self.helper.is_some(), |el| {
                el.child(
                    div()
                        .px(px(tokens.horizontal_padding))
                        .mt(gap)
                        .child(supporting),
                )
            })
    }
}

impl TextFieldState {
    /// 估算光标的 x 坐标（对前缀文本做排版测宽）。
    fn caret_x(&self, window: &mut Window, cx: &mut Context<Self>) -> Pixels {
        let theme = cx.theme();
        let prefix: String = self.value.chars().take(self.caret).collect();
        if prefix.is_empty() {
            return px(0.);
        }
        let font = gpui::Font {
            family: theme.font_family().clone(),
            ..gpui::Font::default()
        };
        let font_size = theme.typography().body_large.size;
        let run = gpui::TextRun {
            len: prefix.len(),
            font,
            color: gpui::black(),
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let text: SharedString = prefix.into();
        let layout = window
            .text_system()
            .layout_line(&text, font_size, &[run], None);
        layout.width
    }

    /// 处理按键；返回是否已处理（用于阻断冒泡）。
    fn handle_key(
        &mut self,
        keystroke: &gpui::Keystroke,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        match (keystroke.key.as_str(), keystroke.key_char.clone()) {
            ("enter", _) => {
                if let Some(handler) = self.on_submit.clone() {
                    let value = self.value.clone();
                    handler(&value, window, cx);
                }
                true
            }
            ("backspace", _) => {
                self.backspace(window, cx);
                true
            }
            ("delete", _) => {
                self.delete(window, cx);
                true
            }
            ("left", _) => {
                self.caret = self.caret.saturating_sub(1);
                cx.notify();
                true
            }
            ("right", _) => {
                self.caret = (self.caret + 1).min(self.value.chars().count());
                cx.notify();
                true
            }
            ("home", _) => {
                self.caret = 0;
                cx.notify();
                true
            }
            ("end", _) => {
                self.caret = self.value.chars().count();
                cx.notify();
                true
            }
            (key, Some(ch))
                if !keystroke.modifiers.control
                    && !keystroke.modifiers.alt
                    && !keystroke.modifiers.platform =>
            {
                let _ = key;
                if let Some(ch) = ch.chars().next()
                    && !ch.is_control()
                {
                    self.insert_char(ch, window, cx);
                    return true;
                }
                false
            }
            _ => false,
        }
    }
}
