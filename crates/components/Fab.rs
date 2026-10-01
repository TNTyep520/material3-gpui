//! MD3 FAB（对应 material-web 的 `md-fab`）。
//!
//! 尺寸：Small 40dp / Standard 56dp / Large 96dp；
//! 颜色：Surface / Primary / Secondary / Tertiary；
//! 支持 Extended FAB（带文字标签）。
//!
//! 交互行为移植自 [m3fx](https://github.com/Glavo/m3fx) 的
//! `M3FloatingActionButtonSkin`（Apache-2.0，© 2026 Glavo）。
//!
//! ```ignore
//! Fab::new("add", IconName::Add)
//!     .label("New item")
//!     .on_click(|_, _, _| {})
//!     .build(cx)   // -> Entity<FabState>
//! ```

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

/// FAB 尺寸。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FabSize {
    /// 40dp。
    Small,
    /// 56dp（默认）。
    #[default]
    Standard,
    /// 80dp。
    Medium,
    /// 96dp。
    Large,
}

/// FAB 配色。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FabColor {
    /// 表面配色。
    Surface,
    /// 主色配色（默认）。
    #[default]
    Primary,
    /// 次级配色。
    Secondary,
    /// 第三配色。
    Tertiary,
}

/// MD3 悬浮操作按钮构建器（`.build(cx)` 产出 [`FabState`]）。
pub struct Fab {
    id: ElementId,
    icon: IconName,
    size: FabSize,
    color: FabColor,
    label: Option<SharedString>,
    lowered: bool,
    on_click: Option<ClickHandler>,
}

/// FAB 的有状态部分。
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
    /// 创建 FAB 构建器。
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

    /// 设置尺寸。
    pub fn size(mut self, size: FabSize) -> Self {
        self.size = size;
        self
    }

    /// 设置配色。
    pub fn color(mut self, color: FabColor) -> Self {
        self.color = color;
        self
    }

    /// Extended FAB：附带文字标签（固定高 56dp）。
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// 设置扩展 FAB 的文字内容。
    pub fn text(self, text: impl Into<SharedString>) -> Self {
        self.label(text)
    }

    /// 降低的 elevation（Level1 而非 Level3）。
    pub fn lowered(mut self, lowered: bool) -> Self {
        self.lowered = lowered;
        self
    }

    /// 设置点击回调。
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    /// 构建有状态组件实体。
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
            /// 创建指定尺寸的 FAB。
            pub fn new(id: impl Into<ElementId>, icon: IconName) -> Self {
                Self(Fab::new(id, icon).size(FabSize::$size))
            }

            /// 设置容器配色。
            pub fn color(mut self, color: FabColor) -> Self {
                self.0 = self.0.color(color);
                self
            }

            /// 设置点击回调。
            pub fn on_click(
                mut self,
                handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
            ) -> Self {
                self.0 = self.0.on_click(handler);
                self
            }

            /// 构建可渲染的 FAB 实体。
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
            /// 创建带文字内容的 FAB。
            pub fn new(
                id: impl Into<ElementId>,
                icon: IconName,
                text: impl Into<SharedString>,
            ) -> Self {
                Self(Fab::new(id, icon).size(FabSize::$size).text(text))
            }

            /// 设置容器配色。
            pub fn color(mut self, color: FabColor) -> Self {
                self.0 = self.0.color(color);
                self
            }

            /// 设置点击回调。
            pub fn on_click(
                mut self,
                handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
            ) -> Self {
                self.0 = self.0.on_click(handler);
                self
            }

            /// 构建可渲染的扩展 FAB 实体。
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
    /// MD3 FAB 样式。
    #[derive(Clone, Debug)]
    pub struct FabStyle {
        /// 容器色。
        pub container_color: gpui::Hsla,
        /// 内容色。
        pub content_color: gpui::Hsla,
        /// 容器边长（extended 时为高度）。
        pub size: Pixels,
        /// 圆角。
        pub corner_radius: Pixels,
        /// 图标尺寸。
        pub icon_size: Pixels,
        /// 图标与文字间距。
        pub icon_gap: Pixels,
        /// 阴影等级。
        pub elevation: Elevation,
        /// 阴影颜色。
        pub shadow_color: gpui::Hsla,
        /// 状态层/涟漪基色。
        pub state_layer_color: gpui::Hsla,
        /// 按压档状态层不透明度。
        pub state_layer_opacity: f32,
        /// extended（带文字）时的水平内边距。
        pub extended_padding: (Pixels, Pixels),
        /// 文字字型。
        pub label: crate::theme::TypeStyle,
    }
    impl FabStyle {
        /// 由令牌推导默认样式。
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
