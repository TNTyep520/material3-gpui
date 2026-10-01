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
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/skins/M3CheckBoxSkin.java

use std::rc::Rc;
use std::time::Instant;

use gpui::{
    App, AppContext as _, Context, ElementId, Entity, InteractiveElement as _, IntoElement,
    ParentElement as _, Render, StatefulInteractiveElement as _, Styled, Window, div,
    prelude::FluentBuilder as _,
};

use crate::icon::{Icon, IconName};
use crate::interaction::InteractiveSurface;
use crate::motion::{Animatable, AnimatedComponent, AnimationDriver, MotionRole, lerp_color};
use crate::theme::ActiveTheme;

type ChangeHandler = Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToggleableState {
    #[default]
    Off,

    On,

    Indeterminate,
}

pub struct Checkbox {
    id: ElementId,
    checked: bool,
    indeterminate: bool,
    disabled: bool,
    error: bool,
    on_change: Option<ChangeHandler>,
}

pub struct CheckboxState {
    id: ElementId,
    checked: bool,
    indeterminate: bool,
    disabled: bool,
    error: bool,
    on_change: Option<ChangeHandler>,

    progress: Animatable,
    surface: InteractiveSurface,
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            checked: false,
            indeterminate: false,
            disabled: false,
            error: false,
            on_change: None,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self.indeterminate = false;
        self
    }

    pub fn toggleable_state(mut self, state: ToggleableState) -> Self {
        self.checked = state == ToggleableState::On;
        self.indeterminate = state == ToggleableState::Indeterminate;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn enabled(self, enabled: bool) -> Self {
        self.disabled(!enabled)
    }

    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
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

    pub fn build(self, cx: &mut App) -> Entity<CheckboxState> {
        let checked = self.checked;
        cx.new(|_| CheckboxState {
            id: self.id,
            checked,
            indeterminate: self.indeterminate,
            disabled: self.disabled,
            error: self.error,
            on_change: self.on_change,
            progress: Animatable::new(
                if checked || self.indeterminate {
                    1.0
                } else {
                    0.0
                },
                1.0e-3,
            ),
            surface: InteractiveSurface::new(),
        })
    }
}

pub struct TriStateCheckbox(Checkbox);

impl TriStateCheckbox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self(Checkbox::new(id))
    }

    pub fn state(mut self, state: ToggleableState) -> Self {
        self.0 = self.0.toggleable_state(state);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.0 = self.0.enabled(enabled);
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.0 = self.0.on_change(move |_, window, cx| handler(window, cx));
        self
    }

    pub fn build(self, cx: &mut App) -> Entity<CheckboxState> {
        self.0.build(cx)
    }
}

impl CheckboxState {
    pub fn checked(&self) -> bool {
        self.checked
    }

    pub fn toggleable_state(&self) -> ToggleableState {
        if self.indeterminate {
            ToggleableState::Indeterminate
        } else if self.checked {
            ToggleableState::On
        } else {
            ToggleableState::Off
        }
    }

    pub fn set_checked(&mut self, checked: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.checked == checked && !self.indeterminate {
            return;
        }
        self.checked = checked;
        self.indeterminate = false;
        let spec = *cx.theme().motion().spec(MotionRole::DefaultEffects);
        self.progress
            .animate_to(if checked { 1.0 } else { 0.0 }, &spec, Instant::now());
        if self.progress.is_running() {
            self.schedule_next(window, cx);
        }
        cx.notify();
    }
}

impl AnimatedComponent for CheckboxState {
    fn step(&mut self, now: Instant) -> bool {
        let running = self.progress.tick(now);
        let surface_running = self.surface.step(now);
        running || surface_running
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        self.surface.driver_mut()
    }
}

impl Render for CheckboxState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.progress.is_running() || self.surface.is_animating() {
            self.schedule_next(window, cx);
        }

        let theme = cx.theme();
        let colors = theme.colors();
        let state_layer = *theme.state_layer();
        let disabled = self.disabled;
        let p = self.progress.value() as f32;
        let style = CheckboxStyle::resolve(theme.token_set(), self.error, self.disabled);

        let accent = if self.error {
            colors.error
        } else {
            colors.primary
        };
        let on_accent = if self.error {
            colors.on_error
        } else {
            colors.on_primary
        };
        let outline = if self.error {
            colors.error
        } else {
            colors.on_surface_variant
        };

        let layer = if self.checked || self.indeterminate {
            accent
        } else {
            colors.on_surface
        };

        let (box_bg, box_border, mark_color) = if disabled {
            if self.checked || self.indeterminate {
                (
                    Some(colors.disabled_content(&state_layer)),
                    None,
                    Some(colors.surface),
                )
            } else {
                (None, Some(colors.disabled_content(&state_layer)), None)
            }
        } else {
            let bg = lerp_color(gpui::Hsla::transparent_black(), accent, p);
            let border = lerp_color(outline, accent, p);
            let mark = lerp_color(gpui::Hsla::transparent_black(), on_accent, p);
            (Some(bg), Some(border), Some(mark))
        };

        let entity = cx.entity();
        let base = div()
            .id(self.id.clone())
            .size(style.touch_target)
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .rounded_full()
            .when(!disabled, |el| el.cursor_pointer().overflow_hidden());

        let base = if disabled {
            base
        } else {
            crate::interaction::wire(
                &self.surface,
                base,
                &entity,
                theme.motion(),
                |s: &mut Self| &mut s.surface,
                layer,
                state_layer.pressed,
                gpui::px(999.),
            )
        };

        let base = if disabled {
            base
        } else {
            let toggle_entity = entity;
            base.on_click(move |_event, window, cx| {
                toggle_entity.update(cx, |state, cx| {
                    let next = !state.checked;
                    state.set_checked(next, window, cx);
                    if let Some(handler) = state.on_change.clone() {
                        handler(next, window, cx);
                    }
                });
            })
        };

        base.child(
            div()
                .size(style.box_size)
                .flex()
                .flex_none()
                .items_center()
                .justify_center()
                .rounded(style.corner_radius)
                .when_some(box_bg, |el, bg| el.bg(bg))
                .when_some(box_border, |el, color| {
                    el.border(style.border_width).border_color(color)
                })
                .when_some(mark_color.filter(|_| p > 0.0), |el, color| {
                    el.child(
                        Icon::new(if self.indeterminate {
                            IconName::Remove
                        } else {
                            IconName::Check
                        })
                        .size(style.mark_size)
                        .color(color.opacity(p)),
                    )
                }),
        )
    }
}

pub use appearance::CheckboxStyle;

mod appearance {
    use crate::theme::TokenSet;
    use gpui::{Hsla, Pixels, px};

    #[derive(Clone, Copy, Debug)]
    pub struct CheckboxStyle {
        pub accent: Hsla,

        pub on_accent: Hsla,

        pub outline: Hsla,

        pub box_size: Pixels,

        pub corner_radius: Pixels,

        pub border_width: Pixels,

        pub mark_size: Pixels,

        pub touch_target: Pixels,

        pub state_layer_color: Hsla,

        pub state_layer_opacity: f32,

        pub disabled_content: Hsla,
    }
    impl CheckboxStyle {
        pub fn resolve(tokens: &TokenSet, error: bool, disabled: bool) -> Self {
            let colors = &tokens.colors;
            let state = &tokens.state_layer;
            let (accent, on_accent, outline) = if error {
                (colors.error, colors.on_error, colors.error)
            } else {
                (colors.primary, colors.on_primary, colors.on_surface_variant)
            };
            Self {
                accent,
                on_accent,
                outline,
                box_size: px(18.),
                corner_radius: px(2.),
                border_width: px(2.),
                mark_size: px(16.),
                touch_target: px(40.),
                state_layer_color: accent,
                state_layer_opacity: state.pressed,
                disabled_content: if disabled {
                    colors.disabled_content(state)
                } else {
                    colors.on_surface
                },
            }
        }
    }
}
