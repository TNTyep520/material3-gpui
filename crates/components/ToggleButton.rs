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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Button.kt

use std::rc::Rc;
use std::time::Instant;

use gpui::{
    App, AppContext as _, Context, ElementId, Entity, Hsla, InteractiveElement as _, IntoElement,
    ParentElement as _, Pixels, Render, SharedString, StatefulInteractiveElement as _, Styled,
    Window, div, prelude::*, px,
};

use crate::icon::{Icon, IconName};
use crate::interaction::InteractiveSurface;
use crate::motion::{AnimatedComponent, AnimationDriver};
use crate::theme::{ActiveTheme, TokenSet};
use crate::tokens::SmallIconButtonTokens;

type ChangeHandler = Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>;
type StyleOverride = Rc<dyn Fn(&mut ToggleButtonStyle)>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToggleButtonVariant {
    #[default]
    Filled,

    Elevated,

    FilledTonal,

    Outlined,
}

#[derive(Clone, Debug)]
pub struct ToggleButtonStyle {
    pub container_color: Option<Hsla>,

    pub outline_color: Option<Hsla>,

    pub content_color: Hsla,

    pub height: Pixels,

    pub horizontal_padding: Pixels,

    pub gap: Pixels,

    pub corner_radius: Pixels,

    pub icon_size: Pixels,
}

impl ToggleButtonStyle {
    pub fn resolve(tokens: &TokenSet, checked: bool) -> Self {
        Self::resolve_variant(tokens, ToggleButtonVariant::Filled, checked)
    }

    pub fn resolve_variant(tokens: &TokenSet, variant: ToggleButtonVariant, checked: bool) -> Self {
        let colors = &tokens.colors;
        let (container_color, content_color, outline_color) = match (variant, checked) {
            (ToggleButtonVariant::Filled, true) => (Some(colors.primary), colors.on_primary, None),
            (ToggleButtonVariant::Filled, false) => (
                Some(colors.surface_container),
                colors.on_surface_variant,
                None,
            ),
            (ToggleButtonVariant::Elevated, true) => (
                Some(colors.primary_container),
                colors.on_primary_container,
                None,
            ),
            (ToggleButtonVariant::Elevated, false) => {
                (Some(colors.surface_container_low), colors.on_surface, None)
            }
            (ToggleButtonVariant::FilledTonal, true) => (
                Some(colors.secondary_container),
                colors.on_secondary_container,
                None,
            ),
            (ToggleButtonVariant::FilledTonal, false) => (
                Some(colors.surface_container),
                colors.on_surface_variant,
                None,
            ),
            (ToggleButtonVariant::Outlined, true) => (
                Some(colors.inverse_surface),
                colors.inverse_on_surface,
                None,
            ),
            (ToggleButtonVariant::Outlined, false) => (
                None,
                colors.on_surface_variant,
                Some(colors.outline_variant),
            ),
        };
        Self {
            container_color,
            outline_color,
            content_color,
            height: SmallIconButtonTokens::CONTAINER_HEIGHT.pixels(),

            horizontal_padding: px(16.),
            gap: px(8.),
            corner_radius: tokens.shapes.full,

            icon_size: px(18.),
        }
    }
}

pub struct ToggleButton {
    id: ElementId,
    label: SharedString,
    icon: Option<IconName>,
    checked: bool,
    disabled: bool,
    variant: ToggleButtonVariant,
    on_change: Option<ChangeHandler>,
    style_override: Option<StyleOverride>,
}

pub struct ToggleButtonState {
    id: ElementId,
    label: SharedString,
    icon: Option<IconName>,
    checked: bool,
    disabled: bool,
    variant: ToggleButtonVariant,
    on_change: Option<ChangeHandler>,
    style_override: Option<StyleOverride>,
    surface: InteractiveSurface,
}

impl ToggleButton {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            checked: false,
            disabled: false,
            variant: ToggleButtonVariant::default(),
            on_change: None,
            style_override: None,
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
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

    pub fn variant(mut self, variant: ToggleButtonVariant) -> Self {
        self.variant = variant;
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

    pub fn style(mut self, style_override: impl Fn(&mut ToggleButtonStyle) + 'static) -> Self {
        self.style_override = Some(Rc::new(style_override));
        self
    }

    pub fn build(self, cx: &mut App) -> Entity<ToggleButtonState> {
        cx.new(|_| ToggleButtonState {
            id: self.id,
            label: self.label,
            icon: self.icon,
            checked: self.checked,
            disabled: self.disabled,
            variant: self.variant,
            on_change: self.on_change,
            style_override: self.style_override,
            surface: InteractiveSurface::new(),
        })
    }
}

macro_rules! toggle_button_variant {
    ($name:ident, $variant:ident) => {
        #[doc = concat!("AndroidX ", stringify!($name), " 对应的切换按钮。")]
        pub struct $name(ToggleButton);

        impl $name {
            pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
                Self(ToggleButton::new(id, label).variant(ToggleButtonVariant::$variant))
            }

            pub fn icon(mut self, icon: IconName) -> Self {
                self.0 = self.0.icon(icon);
                self
            }

            pub fn checked(mut self, checked: bool) -> Self {
                self.0 = self.0.checked(checked);
                self
            }

            pub fn enabled(mut self, enabled: bool) -> Self {
                self.0 = self.0.enabled(enabled);
                self
            }

            pub fn on_checked_change(
                mut self,
                handler: impl Fn(bool, &mut Window, &mut App) + 'static,
            ) -> Self {
                self.0 = self.0.on_checked_change(handler);
                self
            }

            pub fn build(self, cx: &mut App) -> Entity<ToggleButtonState> {
                self.0.build(cx)
            }
        }
    };
}

toggle_button_variant!(ElevatedToggleButton, Elevated);
toggle_button_variant!(FilledTonalToggleButton, FilledTonal);
toggle_button_variant!(OutlinedToggleButton, Outlined);

impl ToggleButtonState {
    pub fn checked(&self) -> bool {
        self.checked
    }

    pub fn set_checked(&mut self, checked: bool, cx: &mut Context<Self>) {
        if self.checked != checked {
            self.checked = checked;
            cx.notify();
        }
    }
}

impl AnimatedComponent for ToggleButtonState {
    fn step(&mut self, now: Instant) -> bool {
        self.surface.step(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        self.surface.driver_mut()
    }
}

impl Render for ToggleButtonState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.surface.is_animating() {
            self.schedule_next(window, cx);
        }

        let theme = cx.theme();
        let colors = theme.colors();
        let state_layer = *theme.state_layer();
        let disabled = self.disabled;
        let checked = self.checked;

        let mut style =
            ToggleButtonStyle::resolve_variant(theme.token_set(), self.variant, checked);
        if let Some(style_override) = &self.style_override {
            style_override(&mut style);
        }

        let foreground = if disabled {
            colors.disabled_content(&state_layer)
        } else {
            style.content_color
        };
        let background = if disabled {
            style
                .container_color
                .map(|_| colors.disabled_container(&state_layer))
        } else {
            style.container_color
        };

        let base = div()
            .id(self.id.clone())
            .h(style.height)
            .px(style.horizontal_padding)
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .gap(style.gap)
            .rounded(style.corner_radius)
            .text_color(foreground)
            .when_some(background, |el, bg_color| el.bg(bg_color))
            .when_some(style.outline_color, |el, outline| {
                el.border_1().border_color(outline)
            })
            .when(!disabled, |el| el.cursor_pointer().overflow_hidden());

        let entity = cx.entity();
        let base = if disabled {
            base
        } else {
            crate::interaction::wire(
                &self.surface,
                base,
                &entity,
                theme.motion(),
                |s: &mut Self| &mut s.surface,
                foreground,
                state_layer.pressed,
                style.corner_radius,
            )
        };

        let base = if disabled {
            base
        } else {
            base.on_click(move |_, window, cx| {
                let change = entity.update(cx, |state, cx| {
                    state.checked = !state.checked;
                    cx.notify();
                    state
                        .on_change
                        .clone()
                        .map(|handler| (handler, state.checked))
                });
                if let Some((handler, checked)) = change {
                    handler(checked, window, cx);
                }
            })
        };

        base.when_some(self.icon.clone(), |el, icon| {
            el.child(Icon::new(icon).size(style.icon_size))
        })
        .child(self.label.clone())
    }
}
