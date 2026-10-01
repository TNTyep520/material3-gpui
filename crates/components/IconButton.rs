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
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/skins/M3IconButtonSkin.java
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/skins/M3IconToggleButtonSkin.java

use std::rc::Rc;
use std::time::Instant;

use gpui::{
    App, AppContext as _, ClickEvent, Context, ElementId, Entity, Hsla, InteractiveElement as _,
    IntoElement, ParentElement as _, Render, StatefulInteractiveElement as _, Styled, Window, div,
    prelude::FluentBuilder as _,
};

use crate::icon::{Icon, IconName};
use crate::interaction::InteractiveSurface;
use crate::motion::{AnimatedComponent, AnimationDriver};
use crate::theme::{ActiveTheme, TokenSet};

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;
type CheckedHandler = Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IconButtonSize {
    XSmall,

    #[default]
    Small,

    Medium,

    Large,

    XLarge,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IconButtonShape {
    #[default]
    Round,

    Square,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IconButtonVariant {
    #[default]
    Standard,

    Filled,

    FilledTonal,

    Outlined,
}

#[derive(Clone, Copy, Debug)]
pub struct IconButtonColors {
    pub container: Option<Hsla>,

    pub content: Hsla,

    pub disabled_container: Option<Hsla>,

    pub disabled_content: Hsla,
}

#[derive(Clone, Copy, Debug)]
pub struct IconToggleButtonColors {
    pub unselected: IconButtonColors,

    pub checked_container: Option<Hsla>,

    pub checked_content: Hsla,
}

pub struct IconButtonDefaults;

impl IconButtonDefaults {
    pub fn colors(tokens: &TokenSet, variant: IconButtonVariant) -> IconButtonColors {
        let style = IconButtonStyle::resolve(tokens, variant, false);
        IconButtonColors {
            container: style.container_color,
            content: style.content_color,
            disabled_container: style
                .container_color
                .map(|_| tokens.colors.disabled_container(&tokens.state_layer)),
            disabled_content: tokens.colors.disabled_content(&tokens.state_layer),
        }
    }

    pub fn toggle_colors(tokens: &TokenSet, variant: IconButtonVariant) -> IconToggleButtonColors {
        let checked = IconButtonStyle::resolve(tokens, variant, true);
        IconToggleButtonColors {
            unselected: Self::colors(tokens, variant),
            checked_container: checked.container_color,
            checked_content: checked.content_color,
        }
    }
}

pub struct IconButton {
    id: ElementId,
    icon: IconName,
    variant: IconButtonVariant,
    selected: bool,
    disabled: bool,
    size: IconButtonSize,
    shape: IconButtonShape,
    colors: Option<IconButtonColors>,
    toggle_colors: Option<IconToggleButtonColors>,
    on_click: Option<ClickHandler>,
    on_checked_change: Option<CheckedHandler>,
}

pub struct IconButtonState {
    id: ElementId,
    icon: IconName,
    variant: IconButtonVariant,
    selected: bool,
    disabled: bool,
    size: IconButtonSize,
    shape: IconButtonShape,
    colors: Option<IconButtonColors>,
    toggle_colors: Option<IconToggleButtonColors>,
    on_click: Option<ClickHandler>,
    on_checked_change: Option<CheckedHandler>,
    surface: InteractiveSurface,
}

impl IconButton {
    pub fn new(id: impl Into<ElementId>, icon: IconName) -> Self {
        Self {
            id: id.into(),
            icon,
            variant: IconButtonVariant::default(),
            selected: false,
            disabled: false,
            size: IconButtonSize::default(),
            shape: IconButtonShape::default(),
            colors: None,
            toggle_colors: None,
            on_click: None,
            on_checked_change: None,
        }
    }

    pub fn variant(mut self, variant: IconButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn filled(self) -> Self {
        self.variant(IconButtonVariant::Filled)
    }

    pub fn tonal(self) -> Self {
        self.variant(IconButtonVariant::FilledTonal)
    }

    pub fn outlined(self) -> Self {
        self.variant(IconButtonVariant::Outlined)
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn size(mut self, size: IconButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn shape(mut self, shape: IconButtonShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn colors(mut self, colors: IconButtonColors) -> Self {
        self.colors = Some(colors);
        self
    }

    pub fn toggle_colors(mut self, colors: IconToggleButtonColors) -> Self {
        self.toggle_colors = Some(colors);
        self
    }

    pub fn enabled(self, enabled: bool) -> Self {
        self.disabled(!enabled)
    }

    pub fn on_checked_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_checked_change = Some(Rc::new(handler));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    pub fn build(self, cx: &mut App) -> Entity<IconButtonState> {
        cx.new(|_| IconButtonState {
            id: self.id,
            icon: self.icon,
            variant: self.variant,
            selected: self.selected,
            disabled: self.disabled,
            size: self.size,
            shape: self.shape,
            colors: self.colors,
            toggle_colors: self.toggle_colors,
            on_click: self.on_click,
            on_checked_change: self.on_checked_change,
            surface: InteractiveSurface::new(),
        })
    }
}

pub struct IconToggleButton(IconButton);

impl IconToggleButton {
    pub fn new(id: impl Into<ElementId>, icon: IconName) -> Self {
        Self(IconButton::new(id, icon))
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.0 = self.0.selected(checked);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.0 = self.0.disabled(!enabled);
        self
    }

    pub fn size(mut self, size: IconButtonSize) -> Self {
        self.0 = self.0.size(size);
        self
    }

    pub fn shape(mut self, shape: IconButtonShape) -> Self {
        self.0 = self.0.shape(shape);
        self
    }

    pub fn colors(mut self, colors: IconToggleButtonColors) -> Self {
        self.0 = self.0.toggle_colors(colors);
        self
    }

    pub fn on_checked_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.0 = self.0.on_checked_change(handler);
        self
    }

    pub fn build(self, cx: &mut App) -> Entity<IconButtonState> {
        self.0.build(cx)
    }
}

macro_rules! icon_button_family {
    ($button:ident, $toggle:ident, $variant:ident) => {
        #[doc = concat!("AndroidX ", stringify!($button), " 变体。")]
        pub struct $button(IconButton);

        impl $button {
            pub fn new(id: impl Into<ElementId>, icon: IconName) -> Self {
                Self(IconButton::new(id, icon).variant(IconButtonVariant::$variant))
            }

            pub fn enabled(mut self, enabled: bool) -> Self {
                self.0 = self.0.disabled(!enabled);
                self
            }

            pub fn size(mut self, size: IconButtonSize) -> Self {
                self.0 = self.0.size(size);
                self
            }

            pub fn shape(mut self, shape: IconButtonShape) -> Self {
                self.0 = self.0.shape(shape);
                self
            }

            pub fn colors(mut self, colors: IconButtonColors) -> Self {
                self.0 = self.0.colors(colors);
                self
            }

            pub fn on_click(
                mut self,
                handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
            ) -> Self {
                self.0 = self.0.on_click(handler);
                self
            }

            pub fn build(self, cx: &mut App) -> Entity<IconButtonState> {
                self.0.build(cx)
            }
        }

        #[doc = concat!("AndroidX ", stringify!($toggle), " 切换变体。")]
        pub struct $toggle(IconToggleButton);

        impl $toggle {
            pub fn new(id: impl Into<ElementId>, icon: IconName) -> Self {
                Self(IconToggleButton(
                    IconButton::new(id, icon).variant(IconButtonVariant::$variant),
                ))
            }

            pub fn checked(mut self, checked: bool) -> Self {
                self.0 = self.0.checked(checked);
                self
            }

            pub fn enabled(mut self, enabled: bool) -> Self {
                self.0 = self.0.enabled(enabled);
                self
            }

            pub fn size(mut self, size: IconButtonSize) -> Self {
                self.0 = self.0.size(size);
                self
            }

            pub fn shape(mut self, shape: IconButtonShape) -> Self {
                self.0 = self.0.shape(shape);
                self
            }

            pub fn colors(mut self, colors: IconToggleButtonColors) -> Self {
                self.0 = self.0.colors(colors);
                self
            }

            pub fn on_checked_change(
                mut self,
                handler: impl Fn(bool, &mut Window, &mut App) + 'static,
            ) -> Self {
                self.0 = self.0.on_checked_change(handler);
                self
            }

            pub fn build(self, cx: &mut App) -> Entity<IconButtonState> {
                self.0.build(cx)
            }
        }
    };
}

icon_button_family!(FilledIconButton, FilledIconToggleButton, Filled);
icon_button_family!(
    FilledTonalIconButton,
    FilledTonalIconToggleButton,
    FilledTonal
);
icon_button_family!(OutlinedIconButton, OutlinedIconToggleButton, Outlined);

impl IconButtonState {
    pub fn checked(&self) -> bool {
        self.selected
    }

    pub fn set_checked(&mut self, checked: bool, cx: &mut Context<Self>) {
        if self.selected != checked {
            self.selected = checked;
            cx.notify();
        }
    }

    pub fn bounds(&self) -> gpui::Bounds<gpui::Pixels> {
        self.surface.bounds.get()
    }
}

impl AnimatedComponent for IconButtonState {
    fn step(&mut self, now: Instant) -> bool {
        self.surface.step(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        self.surface.driver_mut()
    }
}

impl Render for IconButtonState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.surface.is_animating() {
            self.schedule_next(window, cx);
        }

        let theme = cx.theme();
        let colors = theme.colors();
        let state_layer = *theme.state_layer();
        let disabled = self.disabled;
        let selected = self.selected;

        let style = IconButtonStyle::resolve_with_size(
            theme.token_set(),
            self.variant,
            selected,
            self.size,
            self.shape,
        );
        let custom = self
            .toggle_colors
            .map(|toggle| {
                if selected {
                    IconButtonColors {
                        container: toggle.checked_container,
                        content: toggle.checked_content,
                        ..toggle.unselected
                    }
                } else {
                    toggle.unselected
                }
            })
            .or(self.colors);
        let (bg, fg) = if let Some(custom) = custom {
            if disabled {
                (custom.disabled_container, custom.disabled_content)
            } else {
                (custom.container, custom.content)
            }
        } else if disabled {
            (
                style
                    .container_color
                    .map(|_| colors.disabled_container(&state_layer)),
                colors.disabled_content(&state_layer),
            )
        } else {
            (style.container_color, style.content_color)
        };

        let base = div()
            .id(self.id.clone())
            .size(style.size)
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .rounded(style.corner_radius)
            .text_color(fg)
            .when_some(bg, |el, bg_color| el.bg(bg_color))
            .when_some(style.outline_color, |el, outline| {
                el.border_1().border_color(if disabled {
                    colors.on_surface.opacity(state_layer.disabled_container)
                } else {
                    outline
                })
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
                fg,
                state_layer.pressed,
                style.corner_radius,
            )
        };

        let base = if disabled {
            base
        } else if let Some(handler) = self.on_checked_change.clone() {
            base.on_click(cx.listener(move |this, _event, window, cx| {
                let checked = !this.selected;
                this.set_checked(checked, cx);
                handler(checked, window, cx);
            }))
        } else if let Some(handler) = self.on_click.clone() {
            base.on_click(move |event, window, cx| handler(event, window, cx))
        } else {
            base
        };

        base.child(Icon::new(self.icon.clone()).size(style.icon_size))
    }
}

pub use appearance::IconButtonStyle;

mod appearance {
    use super::{IconButtonShape, IconButtonSize, IconButtonVariant};
    use crate::theme::TokenSet;
    use crate::tokens::ShapeValue;
    use gpui::{Hsla, Pixels};

    #[derive(Clone, Debug)]
    pub struct IconButtonStyle {
        pub container_color: Option<Hsla>,

        pub content_color: Hsla,

        pub outline_color: Option<Hsla>,

        pub size: Pixels,

        pub icon_size: Pixels,

        pub corner_radius: Pixels,

        pub state_layer_color: Hsla,

        pub state_layer_opacity: f32,

        pub disabled_content_color: Hsla,
    }
    impl IconButtonStyle {
        pub fn resolve(tokens: &TokenSet, variant: IconButtonVariant, selected: bool) -> Self {
            Self::resolve_with_size(
                tokens,
                variant,
                selected,
                IconButtonSize::Small,
                IconButtonShape::Round,
            )
        }

        pub fn resolve_with_size(
            tokens: &TokenSet,
            variant: IconButtonVariant,
            selected: bool,
            size: IconButtonSize,
            shape: IconButtonShape,
        ) -> Self {
            use crate::tokens::{
                FilledIconButtonTokens, FilledTonalIconButtonTokens, LargeIconButtonTokens,
                MediumIconButtonTokens, OutlinedIconButtonTokens, SmallIconButtonTokens,
                StandardIconButtonTokens, XLargeIconButtonTokens, XSmallIconButtonTokens,
            };
            let colors = &tokens.colors;
            let state = &tokens.state_layer;

            let (container, content, outline) = match (variant, selected) {
                (IconButtonVariant::Standard, false) => (
                    None,
                    StandardIconButtonTokens::UNSELECTED_COLOR.resolve(tokens),
                    None,
                ),
                (IconButtonVariant::Standard, true) => (
                    None,
                    StandardIconButtonTokens::SELECTED_COLOR.resolve(tokens),
                    None,
                ),
                (IconButtonVariant::Filled, false) => (
                    Some(FilledIconButtonTokens::UNSELECTED_CONTAINER_COLOR.resolve(tokens)),
                    FilledIconButtonTokens::UNSELECTED_COLOR.resolve(tokens),
                    None,
                ),
                (IconButtonVariant::Filled, true) => (
                    Some(FilledIconButtonTokens::CONTAINER_COLOR.resolve(tokens)),
                    FilledIconButtonTokens::COLOR.resolve(tokens),
                    None,
                ),
                (IconButtonVariant::FilledTonal, false) => (
                    Some(FilledTonalIconButtonTokens::UNSELECTED_CONTAINER_COLOR.resolve(tokens)),
                    FilledTonalIconButtonTokens::UNSELECTED_COLOR.resolve(tokens),
                    None,
                ),
                (IconButtonVariant::FilledTonal, true) => (
                    Some(FilledTonalIconButtonTokens::CONTAINER_COLOR.resolve(tokens)),
                    FilledTonalIconButtonTokens::COLOR.resolve(tokens),
                    None,
                ),
                (IconButtonVariant::Outlined, false) => (
                    None,
                    OutlinedIconButtonTokens::UNSELECTED_COLOR.resolve(tokens),
                    Some(OutlinedIconButtonTokens::OUTLINE_COLOR.resolve(tokens)),
                ),
                (IconButtonVariant::Outlined, true) => (
                    Some(OutlinedIconButtonTokens::SELECTED_CONTAINER_COLOR.resolve(tokens)),
                    OutlinedIconButtonTokens::SELECTED_COLOR.resolve(tokens),
                    None,
                ),
            };

            let (container_size, icon_size, square_shape) = match size {
                IconButtonSize::XSmall => (
                    XSmallIconButtonTokens::CONTAINER_HEIGHT.pixels(),
                    XSmallIconButtonTokens::ICON_SIZE.pixels(),
                    XSmallIconButtonTokens::CONTAINER_SHAPE_SQUARE,
                ),
                IconButtonSize::Small => (
                    SmallIconButtonTokens::CONTAINER_HEIGHT.pixels(),
                    SmallIconButtonTokens::ICON_SIZE.pixels(),
                    SmallIconButtonTokens::CONTAINER_SHAPE_SQUARE,
                ),
                IconButtonSize::Medium => (
                    MediumIconButtonTokens::CONTAINER_HEIGHT.pixels(),
                    MediumIconButtonTokens::ICON_SIZE.pixels(),
                    MediumIconButtonTokens::CONTAINER_SHAPE_SQUARE,
                ),
                IconButtonSize::Large => (
                    LargeIconButtonTokens::CONTAINER_HEIGHT.pixels(),
                    LargeIconButtonTokens::ICON_SIZE.pixels(),
                    LargeIconButtonTokens::CONTAINER_SHAPE_SQUARE,
                ),
                IconButtonSize::XLarge => (
                    XLargeIconButtonTokens::CONTAINER_HEIGHT.pixels(),
                    XLargeIconButtonTokens::ICON_SIZE.pixels(),
                    XLargeIconButtonTokens::CONTAINER_SHAPE_SQUARE,
                ),
            };
            let radius = match shape {
                IconButtonShape::Round => tokens.shapes.full,
                IconButtonShape::Square => match square_shape.resolve(tokens) {
                    ShapeValue::Rounded { top_start, .. } => top_start.pixels(),
                    ShapeValue::Full => container_size / 2.,
                },
            };

            Self {
                container_color: container,
                content_color: content,
                outline_color: outline,
                size: container_size,
                icon_size,
                corner_radius: radius,
                state_layer_color: content,
                state_layer_opacity: state.pressed,
                disabled_content_color: colors.disabled_content(state),
            }
        }
    }
}
