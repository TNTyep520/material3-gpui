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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/TextField.kt
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/OutlinedTextField.kt

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

pub struct TextField {
    id: ElementId,
    label: SharedString,
    value: SharedString,
    helper: Option<SharedString>,
    error: Option<SharedString>,
    leading_icon: Option<IconName>,
    disabled: bool,
    outlined: bool,
    password: bool,
    on_change: Option<ChangeHandler>,
    on_submit: Option<SubmitHandler>,
}

pub struct TextFieldState {
    id: ElementId,
    label: SharedString,
    helper: Option<SharedString>,
    error: Option<SharedString>,
    leading_icon: Option<IconName>,
    disabled: bool,
    outlined: bool,
    password: bool,
    value: String,

    caret: usize,
    focus: FocusHandle,

    focus_progress: Animatable,
    on_change: Option<ChangeHandler>,
    on_submit: Option<SubmitHandler>,
    driver: AnimationDriver,
}

impl TextField {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            value: SharedString::default(),
            helper: None,
            error: None,
            leading_icon: None,
            disabled: false,
            outlined: false,
            password: false,
            on_change: None,
            on_submit: None,
        }
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }

    pub fn helper(mut self, helper: impl Into<SharedString>) -> Self {
        self.helper = Some(helper.into());
        self
    }

    pub fn error(mut self, error: impl Into<SharedString>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub fn leading_icon(mut self, icon: IconName) -> Self {
        self.leading_icon = Some(icon);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn enabled(self, enabled: bool) -> Self {
        self.disabled(!enabled)
    }

    pub fn outlined(mut self) -> Self {
        self.outlined = true;
        self
    }

    pub fn password(mut self, password: bool) -> Self {
        self.password = password;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    pub fn on_value_change(self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change(handler)
    }

    pub fn on_submit(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_submit = Some(Rc::new(handler));
        self
    }

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
            outlined: self.outlined,
            password: self.password,
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

pub struct OutlinedTextField(TextField);

impl OutlinedTextField {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self(TextField::new(id, label).outlined())
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.0 = self.0.value(value);
        self
    }

    pub fn helper(mut self, helper: impl Into<SharedString>) -> Self {
        self.0 = self.0.helper(helper);
        self
    }

    pub fn error(mut self, error: impl Into<SharedString>) -> Self {
        self.0 = self.0.error(error);
        self
    }

    pub fn leading_icon(mut self, icon: IconName) -> Self {
        self.0 = self.0.leading_icon(icon);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.0 = self.0.enabled(enabled);
        self
    }

    pub fn on_value_change(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.0 = self.0.on_value_change(handler);
        self
    }

    pub fn on_submit(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.0 = self.0.on_submit(handler);
        self
    }

    pub fn build(self, cx: &mut App) -> Entity<TextFieldState> {
        self.0.build(cx)
    }
}

impl TextFieldState {
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn set_value(&mut self, value: &str, cx: &mut Context<Self>) {
        self.value = value.to_string();
        self.caret = self.value.chars().count();
        cx.notify();
    }

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
        let display: SharedString = if self.password {
            "•".repeat(self.value.chars().count()).into()
        } else {
            self.value.clone().into()
        };
        let caret_x = if focused {
            f32::from(self.caret_x(&display, window, cx))
        } else {
            0.0
        };

        let theme = cx.theme();
        let colors = theme.colors();
        let tokens = theme.component().text_field;
        let p = self.focus_progress.value() as f32;

        let has_error = self.error.is_some();
        let style = TextFieldStyle::resolve_for_variant(
            theme.token_set(),
            has_error,
            self.disabled,
            self.outlined,
        );
        let accent = if has_error {
            colors.error
        } else {
            colors.primary
        };

        let border_color = if self.disabled || has_error {
            style.border_color
        } else {
            lerp_color(style.border_color, accent, p)
        };

        let floating = p > 0.5 || !self.value.is_empty();
        let label_color = if focused {
            style.focused_label_color
        } else {
            style.label_color
        };
        let label_style = theme.typography();
        let gap = style.supporting_gap;

        let icon_size = style.icon_size;
        let min_h = style.min_height;

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
            .when(self.outlined, |el| el.border_1())
            .when(!self.outlined, |el| el.border_b_1())
            .border_color(border_color)
            .bg(style.container_color)
            .when(!self.disabled, |el| el.cursor_text())
            .when(!self.disabled, |element| {
                element.on_mouse_down(gpui::MouseButton::Left, {
                    let focus = self.focus.clone();
                    move |_event, window, _cx| window.focus(&focus)
                })
            })
            .on_key_down(move |event, window, cx| {
                key_entity.update(cx, |state, cx| {
                    let handled = state.handle_key(&event.keystroke, window, cx);
                    if handled {
                        cx.stop_propagation();
                    }
                });
            })
            .track_focus(&self.focus)
            .when((focused || p > 0.0) && !self.disabled, |el| {
                el.child(
                    div()
                        .absolute()
                        .inset_0()
                        .rounded(theme.shapes().extra_small)
                        .when(self.outlined, |overlay| overlay.border_2())
                        .when(!self.outlined, |overlay| overlay.border_b_2())
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
                el.child(Icon::new(icon).size(icon_size).color(style.icon_color))
            })
            .child(
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
                            .text_color(style.text_color)
                            .child(display),
                    )
                    .when(self.value.is_empty() && !floating, |el| {
                        el.child(
                            div()
                                .absolute()
                                .left_0()
                                .text_size(label_style.body_large.size)
                                .text_color(style.placeholder_color)
                                .child(self.label.clone()),
                        )
                    })
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
    fn caret_x(&self, display: &str, window: &mut Window, cx: &mut Context<Self>) -> Pixels {
        let theme = cx.theme();
        let prefix: String = display.chars().take(self.caret).collect();
        if prefix.is_empty() {
            return px(0.);
        }
        let font = gpui::Font {
            family: theme.font_family().clone(),
            fallbacks: None,
            features: gpui::FontFeatures::default(),
            weight: gpui::FontWeight::NORMAL,
            style: gpui::FontStyle::Normal,
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

    fn handle_key(
        &mut self,
        keystroke: &gpui::Keystroke,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if self.disabled {
            return false;
        }
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

pub use appearance::TextFieldStyle;

mod appearance {
    use crate::theme::TokenSet;
    use gpui::{Hsla, Pixels, px};

    #[derive(Clone, Copy, Debug)]
    pub struct TextFieldStyle {
        pub container_color: Hsla,

        pub border_color: Hsla,

        pub accent: Hsla,

        pub label_color: Hsla,

        pub focused_label_color: Hsla,

        pub text_color: Hsla,

        pub placeholder_color: Hsla,

        pub helper_color: Hsla,

        pub error_color: Hsla,

        pub icon_color: Hsla,

        pub disabled_container_color: Hsla,

        pub min_height: Pixels,

        pub corner_radius: Pixels,

        pub horizontal_padding: Pixels,

        pub vertical_padding: Pixels,

        pub supporting_gap: Pixels,

        pub icon_size: Pixels,

        pub text: crate::theme::TypeStyle,

        pub floating_label: crate::theme::TypeStyle,

        pub supporting_text: crate::theme::TypeStyle,
    }
    impl TextFieldStyle {
        pub fn resolve(tokens: &TokenSet, error: bool, disabled: bool) -> Self {
            Self::resolve_for_variant(tokens, error, disabled, false)
        }

        pub fn resolve_for_variant(
            tokens: &TokenSet,
            error: bool,
            disabled: bool,
            outlined: bool,
        ) -> Self {
            let colors = &tokens.colors;
            let state = &tokens.state_layer;
            let field = &tokens.component.text_field;
            let accent = if error { colors.error } else { colors.primary };
            Self {
                container_color: if disabled {
                    colors.on_surface.opacity(state.disabled_container / 6.0)
                } else if outlined {
                    colors.surface
                } else {
                    colors.surface_container_highest
                },
                border_color: if disabled {
                    colors.disabled_container(state)
                } else if error {
                    colors.error
                } else {
                    colors.outline
                },
                accent,
                label_color: if disabled {
                    colors.disabled_content(state)
                } else if error {
                    colors.error
                } else {
                    colors.on_surface_variant
                },
                focused_label_color: if error { colors.error } else { colors.primary },
                text_color: if disabled {
                    colors.disabled_content(state)
                } else {
                    colors.on_surface
                },
                placeholder_color: if disabled {
                    colors.disabled_content(state)
                } else {
                    colors.on_surface_variant
                },
                helper_color: colors.on_surface_variant,
                error_color: colors.error,
                icon_color: if disabled {
                    colors.disabled_content(state)
                } else {
                    colors.on_surface_variant
                },
                disabled_container_color: colors.on_surface.opacity(state.disabled_container / 6.0),
                min_height: px(field.min_height),
                corner_radius: tokens.shapes.extra_small,
                horizontal_padding: px(field.horizontal_padding),
                vertical_padding: px(field.top_padding),
                supporting_gap: px(field.supporting_gap),
                icon_size: px(field.icon_size),
                text: tokens.typography.body_large,
                floating_label: tokens.typography.label_small,
                supporting_text: tokens.typography.body_small,
            }
        }
    }
}

pub struct SecureTextField {
    id: ElementId,
    label: SharedString,
    value: SharedString,
    enabled: bool,
    on_value_change: Option<ChangeHandler>,
}
impl SecureTextField {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            value: SharedString::default(),
            enabled: true,
            on_value_change: None,
        }
    }
    pub fn value(mut self, v: impl Into<SharedString>) -> Self {
        self.value = v.into();
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    pub fn on_value_change(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }
}
impl SecureTextField {
    pub fn build(self, cx: &mut App) -> Entity<TextFieldState> {
        let mut field = TextField::new(self.id, self.label)
            .password(true)
            .value(self.value)
            .enabled(self.enabled);
        if let Some(handler) = self.on_value_change {
            field = field.on_value_change(move |value, window, cx| handler(value, window, cx));
        }
        field.build(cx)
    }
}
