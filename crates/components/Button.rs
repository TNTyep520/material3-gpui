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
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/skins/M3ButtonSkin.java
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/skins/M3LabeledButtonSkinBase.java

use std::rc::Rc;
use std::time::Instant;

use gpui::{
    App, AppContext as _, ClickEvent, Context, ElementId, Entity, InteractiveElement as _,
    IntoElement, ParentElement as _, Render, SharedString, StatefulInteractiveElement as _, Styled,
    Window, div, prelude::FluentBuilder as _, px,
};

use crate::icon::{Icon, IconName};
use crate::interaction::InteractiveSurface;
use crate::motion::{AnimatedComponent, AnimationDriver};
use crate::theme::{ActiveTheme, Elevation};

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;
type StyleOverride = Box<dyn Fn(&mut ButtonStyle)>;

pub struct Button {
    id: ElementId,
    label: SharedString,
    variant: ButtonVariant,
    leading_icon: Option<IconName>,
    trailing_icon: Option<IconName>,
    disabled: bool,
    on_click: Option<ClickHandler>,
    style_override: Option<StyleOverride>,
}

pub struct ButtonState {
    id: ElementId,
    label: SharedString,
    variant: ButtonVariant,
    leading_icon: Option<IconName>,
    trailing_icon: Option<IconName>,
    disabled: bool,
    on_click: Option<ClickHandler>,
    style_override: Option<StyleOverride>,
    surface: InteractiveSurface,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            variant: ButtonVariant::default(),
            leading_icon: None,
            trailing_icon: None,
            disabled: false,
            on_click: None,
            style_override: None,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn filled(self) -> Self {
        self.variant(ButtonVariant::Filled)
    }

    pub fn outlined(self) -> Self {
        self.variant(ButtonVariant::Outlined)
    }

    pub fn text(self) -> Self {
        self.variant(ButtonVariant::Text)
    }

    pub fn elevated(self) -> Self {
        self.variant(ButtonVariant::Elevated)
    }

    pub fn tonal(self) -> Self {
        self.variant(ButtonVariant::FilledTonal)
    }

    pub fn leading_icon(mut self, icon: IconName) -> Self {
        self.leading_icon = Some(icon);
        self
    }

    pub fn trailing_icon(mut self, icon: IconName) -> Self {
        self.trailing_icon = Some(icon);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn enabled(self, enabled: bool) -> Self {
        self.disabled(!enabled)
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    pub fn style(mut self, override_fn: impl Fn(&mut ButtonStyle) + 'static) -> Self {
        self.style_override = Some(Box::new(override_fn));
        self
    }

    pub fn build(self, cx: &mut App) -> Entity<ButtonState> {
        cx.new(|_| ButtonState {
            id: self.id,
            label: self.label,
            variant: self.variant,
            leading_icon: self.leading_icon,
            trailing_icon: self.trailing_icon,
            disabled: self.disabled,
            on_click: self.on_click,
            style_override: self.style_override,
            surface: InteractiveSurface::new(),
        })
    }
}

macro_rules! button_variant {
    ($name:ident, $variant:ident) => {
        #[doc = concat!("AndroidX ", stringify!($name), " 对应的 GPUI 按钮构建器。")]
        pub struct $name(Button);

        impl $name {
            pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
                Self(Button::new(id, label).variant(ButtonVariant::$variant))
            }

            pub fn enabled(mut self, enabled: bool) -> Self {
                self.0 = self.0.enabled(enabled);
                self
            }

            pub fn leading_icon(mut self, icon: IconName) -> Self {
                self.0 = self.0.leading_icon(icon);
                self
            }

            pub fn trailing_icon(mut self, icon: IconName) -> Self {
                self.0 = self.0.trailing_icon(icon);
                self
            }

            pub fn on_click(
                mut self,
                handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
            ) -> Self {
                self.0 = self.0.on_click(handler);
                self
            }

            pub fn style(mut self, override_fn: impl Fn(&mut ButtonStyle) + 'static) -> Self {
                self.0 = self.0.style(override_fn);
                self
            }

            pub fn build(self, cx: &mut App) -> Entity<ButtonState> {
                self.0.build(cx)
            }
        }
    };
}

button_variant!(ElevatedButton, Elevated);
button_variant!(FilledTonalButton, FilledTonal);
button_variant!(OutlinedButton, Outlined);
button_variant!(TextButton, Text);

impl ButtonState {
    pub fn bounds(&self) -> gpui::Bounds<gpui::Pixels> {
        self.surface.bounds.get()
    }

    pub fn set_on_click(&mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) {
        self.on_click = Some(Rc::new(handler));
    }
}

impl AnimatedComponent for ButtonState {
    fn step(&mut self, now: Instant) -> bool {
        self.surface.step(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        self.surface.driver_mut()
    }
}

impl Render for ButtonState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.surface.is_animating() {
            self.schedule_next(window, cx);
        }

        let has_leading = self.leading_icon.is_some();
        let has_trailing = self.trailing_icon.is_some();

        let mut style = if self.disabled {
            ButtonStyle::resolve_disabled(
                cx.theme().token_set(),
                self.variant,
                has_leading,
                has_trailing,
            )
        } else {
            ButtonStyle::resolve(
                cx.theme().token_set(),
                self.variant,
                has_leading,
                has_trailing,
            )
        };
        if let Some(override_fn) = &self.style_override {
            override_fn(&mut style);
        }

        let (pl, pr) = style.padding;
        let icon_size = style.icon_size;

        let base = div()
            .id(self.id.clone())
            .h(style.height)
            .min_w(px(58.))
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .gap(style.icon_gap)
            .pl(pl)
            .pr(pr)
            .rounded(style.corner_radius)
            .text_color(style.content_color)
            .when(!self.disabled, |el| el.cursor_pointer().overflow_hidden());
        let base = style.label.apply(base);

        let base = if let Some(bg_color) = style.container_color {
            base.bg(bg_color)
        } else {
            base
        };

        let base = if let Some(outline_color) = style.outline_color {
            base.border_1().border_color(outline_color)
        } else {
            base
        };

        let hovered = self.surface.hovered && !self.disabled;
        let elevation = if hovered {
            match self.variant {
                ButtonVariant::Filled => Elevation::Level1,
                ButtonVariant::Elevated => Elevation::Level2,
                _ => style.elevation,
            }
        } else {
            style.elevation
        };
        let base = if elevation != Elevation::Level0 && !self.disabled {
            base.shadow(elevation.shadows(style.shadow_color))
        } else {
            base
        };

        let entity = cx.entity();
        let base = if self.disabled {
            base
        } else {
            crate::interaction::wire(
                &self.surface,
                base,
                &entity,
                cx.theme().motion(),
                |s: &mut Self| &mut s.surface,
                style.state_layer_color,
                style.state_layer_opacity,
                style.corner_radius,
            )
        };

        let base = if self.disabled {
            base
        } else if let Some(handler) = self.on_click.clone() {
            base.on_click(move |event, window, cx| handler(event, window, cx))
        } else {
            base
        };

        base.when_some(self.leading_icon.clone(), |el, icon| {
            el.child(Icon::new(icon).size(icon_size))
        })
        .child(self.label.clone())
        .when_some(self.trailing_icon.clone(), |el, icon| {
            el.child(Icon::new(icon).size(icon_size))
        })
    }
}

pub use appearance::{ButtonStyle, ButtonVariant};

mod appearance {
    use crate::theme::{Elevation, TokenSet};
    use gpui::{Hsla, Pixels, px};

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub enum ButtonVariant {
        #[default]
        Filled,

        Outlined,

        Text,

        Elevated,

        FilledTonal,
    }

    #[derive(Clone, Debug)]
    pub struct ButtonStyle {
        pub container_color: Option<Hsla>,

        pub content_color: Hsla,

        pub outline_color: Option<Hsla>,

        pub elevation: Elevation,

        pub shadow_color: Hsla,

        pub disabled_container_color: Hsla,

        pub disabled_content_color: Hsla,

        pub state_layer_color: Hsla,

        pub state_layer_opacity: f32,

        pub height: Pixels,

        pub corner_radius: Pixels,

        pub padding: (Pixels, Pixels),

        pub icon_size: Pixels,

        pub icon_gap: Pixels,

        pub label: crate::theme::TypeStyle,
    }
    impl ButtonStyle {
        pub fn resolve(
            tokens: &TokenSet,
            variant: ButtonVariant,
            leading_icon: bool,
            trailing_icon: bool,
        ) -> Self {
            Self::resolve_inner(tokens, variant, leading_icon, trailing_icon, false)
        }

        pub fn resolve_disabled(
            tokens: &TokenSet,
            variant: ButtonVariant,
            leading_icon: bool,
            trailing_icon: bool,
        ) -> Self {
            Self::resolve_inner(tokens, variant, leading_icon, trailing_icon, true)
        }

        fn resolve_inner(
            tokens: &TokenSet,
            variant: ButtonVariant,
            leading_icon: bool,
            trailing_icon: bool,
            disabled: bool,
        ) -> Self {
            let colors = &tokens.colors;
            let button = &tokens.component.button;
            let label = tokens.typography.label_large;

            let (container, content, outline, elevation) = match variant {
                ButtonVariant::Filled => (
                    Some(colors.primary),
                    colors.on_primary,
                    None,
                    Elevation::Level0,
                ),
                ButtonVariant::FilledTonal => (
                    Some(colors.secondary_container),
                    colors.on_secondary_container,
                    None,
                    Elevation::Level0,
                ),
                ButtonVariant::Elevated => (
                    Some(colors.surface_container_low),
                    colors.primary,
                    None,
                    Elevation::Level1,
                ),
                ButtonVariant::Outlined => (
                    None,
                    colors.primary,
                    Some(colors.outline),
                    Elevation::Level0,
                ),
                ButtonVariant::Text => (None, colors.primary, None, Elevation::Level0),
            };

            let is_text = variant == ButtonVariant::Text;
            let with_icon = px(button.horizontal_padding_with_icon);
            let plain = px(button.horizontal_padding);
            let text_pad = px(button.text_horizontal_padding);
            let padding = match (is_text, leading_icon, trailing_icon) {
                (true, _, _) => (text_pad, text_pad),
                (false, true, false) => (with_icon, plain),
                (false, false, true) => (plain, with_icon),
                (false, true, true) => (with_icon, with_icon),
                _ => (plain, plain),
            };

            let state = &tokens.state_layer;
            Self {
                container_color: if disabled {
                    container.map(|_| colors.disabled_container(state))
                } else {
                    container
                },
                content_color: if disabled {
                    colors.disabled_content(state)
                } else {
                    content
                },
                outline_color: outline.map(|c| {
                    if disabled {
                        colors.on_surface.opacity(state.disabled_container)
                    } else {
                        c
                    }
                }),
                elevation: if disabled {
                    Elevation::Level0
                } else {
                    elevation
                },
                shadow_color: colors.shadow,
                disabled_container_color: colors.disabled_container(state),
                disabled_content_color: colors.disabled_content(state),
                state_layer_color: content,
                state_layer_opacity: state.pressed,
                height: px(button.height),
                corner_radius: tokens.shapes.full,
                padding,
                icon_size: px(button.icon_size),
                icon_gap: px(button.icon_gap),
                label,
            }
        }
    }
}
