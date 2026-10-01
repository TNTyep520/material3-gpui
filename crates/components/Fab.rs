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
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/skins/M3FloatingActionButtonSkin.java

use std::rc::Rc;
use std::time::Instant;

use gpui::{
    App, AppContext as _, ClickEvent, Context, ElementId, Entity, InteractiveElement as _,
    IntoElement, ParentElement as _, Render, SharedString, StatefulInteractiveElement as _, Styled,
    Window, div, prelude::FluentBuilder as _,
};

use crate::icon::{Icon, IconName};
use crate::interaction::InteractiveSurface;
use crate::motion::{AnimatedComponent, AnimationDriver};
use crate::theme::ActiveTheme;

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FabSize {
    Small,

    #[default]
    Standard,

    Medium,

    Large,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FabColor {
    Surface,

    #[default]
    Primary,

    Secondary,

    Tertiary,
}

pub struct Fab {
    id: ElementId,
    icon: IconName,
    size: FabSize,
    color: FabColor,
    label: Option<SharedString>,
    lowered: bool,
    on_click: Option<ClickHandler>,
}

pub struct FabState {
    id: ElementId,
    icon: IconName,
    size: FabSize,
    color: FabColor,
    label: Option<SharedString>,
    lowered: bool,
    on_click: Option<ClickHandler>,
    surface: InteractiveSurface,
}

impl Fab {
    pub fn new(id: impl Into<ElementId>, icon: IconName) -> Self {
        Self {
            id: id.into(),
            icon,
            size: FabSize::default(),
            color: FabColor::default(),
            label: None,
            lowered: false,
            on_click: None,
        }
    }

    pub fn size(mut self, size: FabSize) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: FabColor) -> Self {
        self.color = color;
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn text(self, text: impl Into<SharedString>) -> Self {
        self.label(text)
    }

    pub fn lowered(mut self, lowered: bool) -> Self {
        self.lowered = lowered;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    pub fn build(self, cx: &mut App) -> Entity<FabState> {
        cx.new(|_| FabState {
            id: self.id,
            icon: self.icon,
            size: self.size,
            color: self.color,
            label: self.label,
            lowered: self.lowered,
            on_click: self.on_click,
            surface: InteractiveSurface::new(),
        })
    }
}

macro_rules! floating_action_button {
    ($name:ident, $size:ident) => {
        #[doc = concat!("AndroidX ", stringify!($name), " 对应的图标 FAB。")]
        pub struct $name(Fab);

        impl $name {
            pub fn new(id: impl Into<ElementId>, icon: IconName) -> Self {
                Self(Fab::new(id, icon).size(FabSize::$size))
            }

            pub fn color(mut self, color: FabColor) -> Self {
                self.0 = self.0.color(color);
                self
            }

            pub fn on_click(
                mut self,
                handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
            ) -> Self {
                self.0 = self.0.on_click(handler);
                self
            }

            pub fn build(self, cx: &mut App) -> Entity<FabState> {
                self.0.build(cx)
            }
        }
    };
}

macro_rules! extended_floating_action_button {
    ($name:ident, $size:ident) => {
        #[doc = concat!("AndroidX ", stringify!($name), " 对应的文字 FAB。")]
        pub struct $name(Fab);

        impl $name {
            pub fn new(
                id: impl Into<ElementId>,
                icon: IconName,
                text: impl Into<SharedString>,
            ) -> Self {
                Self(Fab::new(id, icon).size(FabSize::$size).text(text))
            }

            pub fn color(mut self, color: FabColor) -> Self {
                self.0 = self.0.color(color);
                self
            }

            pub fn on_click(
                mut self,
                handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
            ) -> Self {
                self.0 = self.0.on_click(handler);
                self
            }

            pub fn build(self, cx: &mut App) -> Entity<FabState> {
                self.0.build(cx)
            }
        }
    };
}

floating_action_button!(FloatingActionButton, Standard);
floating_action_button!(SmallFloatingActionButton, Small);
floating_action_button!(MediumFloatingActionButton, Medium);
floating_action_button!(LargeFloatingActionButton, Large);
extended_floating_action_button!(ExtendedFloatingActionButton, Standard);
extended_floating_action_button!(SmallExtendedFloatingActionButton, Small);
extended_floating_action_button!(MediumExtendedFloatingActionButton, Medium);
extended_floating_action_button!(LargeExtendedFloatingActionButton, Large);

impl AnimatedComponent for FabState {
    fn step(&mut self, now: Instant) -> bool {
        self.surface.step(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        self.surface.driver_mut()
    }
}

impl Render for FabState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.surface.is_animating() {
            self.schedule_next(window, cx);
        }

        let theme = cx.theme();
        let state_layer = *theme.state_layer();
        let extended = self.label.is_some();
        let style = FabStyle::resolve(theme.token_set(), self.size, self.color, self.lowered);
        let (bg, fg) = (style.container_color, style.content_color);
        let (container, radius, icon_size) = (style.size, style.corner_radius, style.icon_size);
        let elevation = style.elevation;
        let label_style = style.label;
        let shadow_color = style.shadow_color;

        let base = div()
            .id(self.id.clone())
            .h(container)
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .gap(style.icon_gap)
            .rounded(radius)
            .bg(bg)
            .text_color(fg)
            .shadow(elevation.shadows(shadow_color))
            .cursor_pointer()
            .overflow_hidden();

        let base = if extended {
            label_style.apply(
                base.pl(style.extended_padding.0)
                    .pr(style.extended_padding.1),
            )
        } else {
            base.w(container)
        };

        let entity = cx.entity();
        let base = {
            crate::interaction::wire(
                &self.surface,
                base,
                &entity,
                theme.motion(),
                |s: &mut Self| &mut s.surface,
                fg,
                state_layer.pressed,
                radius,
            )
        };

        let base = if let Some(handler) = self.on_click.clone() {
            base.on_click(move |event, window, cx| handler(event, window, cx))
        } else {
            base
        };

        base.child(Icon::new(self.icon).size(icon_size))
            .when_some(self.label.clone(), |el, label| el.child(label))
    }
}

pub use appearance::FabStyle;

mod appearance {
    use super::FabColor;
    use super::FabSize;
    use crate::theme::{Elevation, TokenSet};
    use gpui::{Pixels, px};

    #[derive(Clone, Debug)]
    pub struct FabStyle {
        pub container_color: gpui::Hsla,

        pub content_color: gpui::Hsla,

        pub size: Pixels,

        pub corner_radius: Pixels,

        pub icon_size: Pixels,

        pub icon_gap: Pixels,

        pub elevation: Elevation,

        pub shadow_color: gpui::Hsla,

        pub state_layer_color: gpui::Hsla,

        pub state_layer_opacity: f32,

        pub extended_padding: (Pixels, Pixels),

        pub label: crate::theme::TypeStyle,
    }
    impl FabStyle {
        pub fn resolve(tokens: &TokenSet, size: FabSize, color: FabColor, lowered: bool) -> Self {
            let colors = &tokens.colors;
            let shapes = tokens.shapes;
            let state = &tokens.state_layer;

            let (container, content) = match color {
                FabColor::Surface => (colors.surface_container_high, colors.primary),
                FabColor::Primary => (colors.primary_container, colors.on_primary_container),
                FabColor::Secondary => (colors.secondary_container, colors.on_secondary_container),
                FabColor::Tertiary => (colors.tertiary_container, colors.on_tertiary_container),
            };
            let (size, radius, icon) = match size {
                FabSize::Small => (px(40.), shapes.medium, px(24.)),
                FabSize::Standard => (px(56.), shapes.large, px(24.)),
                FabSize::Medium => (px(80.), shapes.extra_large, px(28.)),
                FabSize::Large => (px(96.), shapes.extra_large, px(32.)),
            };

            Self {
                container_color: container,
                content_color: content,
                size,
                corner_radius: radius,
                icon_size: icon,
                icon_gap: px(8.),
                elevation: if lowered {
                    Elevation::Level1
                } else {
                    Elevation::Level3
                },
                shadow_color: colors.shadow,
                state_layer_color: content,
                state_layer_opacity: state.pressed,
                extended_padding: (px(16.), px(20.)),
                label: tokens.typography.label_large,
            }
        }
    }
}
