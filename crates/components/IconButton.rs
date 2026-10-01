//! MD3 IconButton（对应 material-web 的 `md-icon-button` 系列）。
//!
//! 变体：Standard / Filled / FilledTonal / Outlined。
//! 规格：容器 40×40dp、图标 24dp、圆形。支持 toggle（选中态）。
//!
//! 交互行为移植自 [m3fx](https://github.com/Glavo/m3fx) 的
//! `M3IconButtonSkin` / `M3IconToggleButtonSkin`（Apache-2.0，© 2026 Glavo）。
//!
//! ```ignore
//! IconButton::new("fav", IconName::Favorite)
//!     .selected(is_fav)
//!     .on_click(|_, _, _| {})
//!     .build(cx)   // -> Entity<IconButtonState>
//! ```

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

/// AndroidX 图标按钮的五档容器尺寸。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IconButtonSize {
    /// 32dp 容器。
    XSmall,
    /// 40dp 容器。
    #[default]
    Small,
    /// 56dp 容器。
    Medium,
    /// 96dp 容器。
    Large,
    /// 136dp 容器。
    XLarge,
}

/// 图标按钮容器形状。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IconButtonShape {
    /// 圆形容器。
    #[default]
    Round,
    /// 方形容器，使用当前尺寸的 Material 圆角令牌。
    Square,
}

/// 图标按钮变体。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IconButtonVariant {
    /// 标准无容器图标按钮。
    #[default]
    Standard,
    /// 实心图标按钮。
    Filled,
    /// 次级色调实心图标按钮。
    FilledTonal,
    /// 描边图标按钮。
    Outlined,
}

/// AndroidX IconButtonColors 对应的普通与禁用配色。
#[derive(Clone, Copy, Debug)]
pub struct IconButtonColors {
    /// 普通容器色；None 表示透明。
    pub container: Option<Hsla>,
    /// 普通图标色。
    pub content: Hsla,
    /// 禁用容器色；None 表示透明。
    pub disabled_container: Option<Hsla>,
    /// 禁用图标色。
    pub disabled_content: Hsla,
}

/// AndroidX IconToggleButtonColors 对应的未选中、选中和禁用配色。
#[derive(Clone, Copy, Debug)]
pub struct IconToggleButtonColors {
    /// 未选中及禁用配色。
    pub unselected: IconButtonColors,
    /// 选中容器色；None 表示透明。
    pub checked_container: Option<Hsla>,
    /// 选中图标色。
    pub checked_content: Hsla,
}

/// 从主题令牌生成 AndroidX 各变体默认配色。
pub struct IconButtonDefaults;

impl IconButtonDefaults {
    /// 解析指定变体的普通按钮配色。
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

    /// 解析指定变体的切换按钮配色。
    pub fn toggle_colors(tokens: &TokenSet, variant: IconButtonVariant) -> IconToggleButtonColors {
        let checked = IconButtonStyle::resolve(tokens, variant, true);
        IconToggleButtonColors {
            unselected: Self::colors(tokens, variant),
            checked_container: checked.container_color,
            checked_content: checked.content_color,
        }
    }
}

/// MD3 图标按钮构建器（`.build(cx)` 产出 [`IconButtonState`]）。
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

/// 图标按钮的有状态部分。
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
    /// 创建图标按钮构建器。
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

    /// 设置变体。
    pub fn variant(mut self, variant: IconButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Filled 变体。
    pub fn filled(self) -> Self {
        self.variant(IconButtonVariant::Filled)
    }

    /// FilledTonal 变体。
    pub fn tonal(self) -> Self {
        self.variant(IconButtonVariant::FilledTonal)
    }

    /// Outlined 变体。
    pub fn outlined(self) -> Self {
        self.variant(IconButtonVariant::Outlined)
    }

    /// toggle 选中态。
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// 设置容器与图标尺寸。
    pub fn size(mut self, size: IconButtonSize) -> Self {
        self.size = size;
        self
    }

    /// 设置圆形或方形容器。
    pub fn shape(mut self, shape: IconButtonShape) -> Self {
        self.shape = shape;
        self
    }

    /// 覆盖普通与禁用配色。
    pub fn colors(mut self, colors: IconButtonColors) -> Self {
        self.colors = Some(colors);
        self
    }

    /// 覆盖未选中、选中和禁用配色。
    pub fn toggle_colors(mut self, colors: IconToggleButtonColors) -> Self {
        self.toggle_colors = Some(colors);
        self
    }

    /// 设置 AndroidX 对应的 enabled 状态。
    pub fn enabled(self, enabled: bool) -> Self {
        self.disabled(!enabled)
    }

    /// 设置切换回调；点击时传递新的选中状态。
    pub fn on_checked_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_checked_change = Some(Rc::new(handler));
        self
    }

    /// 设置禁用态。
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
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

/// 可切换的标准图标按钮；点击后更新 checked 并通知回调。
pub struct IconToggleButton(IconButton);

impl IconToggleButton {
    /// 创建未选中的切换按钮。
    pub fn new(id: impl Into<ElementId>, icon: IconName) -> Self {
        Self(IconButton::new(id, icon))
    }

    /// 设置受控的初始选中状态。
    pub fn checked(mut self, checked: bool) -> Self {
        self.0 = self.0.selected(checked);
        self
    }

    /// 设置启用状态。
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.0 = self.0.disabled(!enabled);
        self
    }

    /// 设置尺寸。
    pub fn size(mut self, size: IconButtonSize) -> Self {
        self.0 = self.0.size(size);
        self
    }

    /// 设置形状。
    pub fn shape(mut self, shape: IconButtonShape) -> Self {
        self.0 = self.0.shape(shape);
        self
    }

    /// 覆盖未选中、选中和禁用配色。
    pub fn colors(mut self, colors: IconToggleButtonColors) -> Self {
        self.0 = self.0.toggle_colors(colors);
        self
    }

    /// 接收每次点击产生的新选中状态。
    pub fn on_checked_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.0 = self.0.on_checked_change(handler);
        self
    }

    /// 构建切换按钮实体。
    pub fn build(self, cx: &mut App) -> Entity<IconButtonState> {
        self.0.build(cx)
    }
}

macro_rules! icon_button_family {
    ($button:ident, $toggle:ident, $variant:ident) => {
        #[doc = concat!("AndroidX ", stringify!($button), " 变体。")]
        pub struct $button(IconButton);

        impl $button {
            /// 创建指定变体的图标按钮。
            pub fn new(id: impl Into<ElementId>, icon: IconName) -> Self {
                Self(IconButton::new(id, icon).variant(IconButtonVariant::$variant))
            }

            /// 设置启用状态。
            pub fn enabled(mut self, enabled: bool) -> Self {
                self.0 = self.0.disabled(!enabled);
                self
            }

            /// 设置尺寸。
            pub fn size(mut self, size: IconButtonSize) -> Self {
                self.0 = self.0.size(size);
                self
            }

            /// 设置形状。
            pub fn shape(mut self, shape: IconButtonShape) -> Self {
                self.0 = self.0.shape(shape);
                self
            }

            /// 覆盖普通与禁用配色。
            pub fn colors(mut self, colors: IconButtonColors) -> Self {
                self.0 = self.0.colors(colors);
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

            /// 构建图标按钮实体。
            pub fn build(self, cx: &mut App) -> Entity<IconButtonState> {
                self.0.build(cx)
            }
        }

        #[doc = concat!("AndroidX ", stringify!($toggle), " 切换变体。")]
        pub struct $toggle(IconToggleButton);

        impl $toggle {
            /// 创建指定变体的切换按钮。
            pub fn new(id: impl Into<ElementId>, icon: IconName) -> Self {
                Self(IconToggleButton(
                    IconButton::new(id, icon).variant(IconButtonVariant::$variant),
                ))
            }

            /// 设置初始选中状态。
            pub fn checked(mut self, checked: bool) -> Self {
                self.0 = self.0.checked(checked);
                self
            }

            /// 设置启用状态。
            pub fn enabled(mut self, enabled: bool) -> Self {
                self.0 = self.0.enabled(enabled);
                self
            }

            /// 设置尺寸。
            pub fn size(mut self, size: IconButtonSize) -> Self {
                self.0 = self.0.size(size);
                self
            }

            /// 设置形状。
            pub fn shape(mut self, shape: IconButtonShape) -> Self {
                self.0 = self.0.shape(shape);
                self
            }

            /// 覆盖未选中、选中和禁用配色。
            pub fn colors(mut self, colors: IconToggleButtonColors) -> Self {
                self.0 = self.0.colors(colors);
                self
            }

            /// 接收每次点击产生的新选中状态。
            pub fn on_checked_change(
                mut self,
                handler: impl Fn(bool, &mut Window, &mut App) + 'static,
            ) -> Self {
                self.0 = self.0.on_checked_change(handler);
                self
            }

            /// 构建切换按钮实体。
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
    /// 返回当前切换状态。
    pub fn checked(&self) -> bool {
        self.selected
    }

    /// 更新切换状态并重新渲染。
    pub fn set_checked(&mut self, checked: bool, cx: &mut Context<Self>) {
        if self.selected != checked {
            self.selected = checked;
            cx.notify();
        }
    }

    /// 组件最近的边界（窗口坐标），可用于菜单等弹层锚定。
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

        base.child(Icon::new(self.icon).size(style.icon_size))
    }
}

pub use appearance::IconButtonStyle;

mod appearance {
    use super::{IconButtonShape, IconButtonSize, IconButtonVariant};
    use crate::theme::TokenSet;
    use crate::tokens::ShapeValue;
    use gpui::{Hsla, Pixels};
    /// MD3 图标按钮样式（对应 button.css 的 `.m3-icon-button` 段落）。
    #[derive(Clone, Debug)]
    pub struct IconButtonStyle {
        /// 容器色（`None` 为透明）。
        pub container_color: Option<Hsla>,
        /// 图标色。
        pub content_color: Hsla,
        /// 描边色（`Some` 启用 1dp 描边）。
        pub outline_color: Option<Hsla>,
        /// 容器边长（正方形）。
        pub size: Pixels,
        /// 图标尺寸。
        pub icon_size: Pixels,
        /// 圆角（圆形）。
        pub corner_radius: Pixels,
        /// 状态层/涟漪基色。
        pub state_layer_color: Hsla,
        /// 按压档状态层不透明度。
        pub state_layer_opacity: f32,
        /// 禁用态内容色。
        pub disabled_content_color: Hsla,
    }
    impl IconButtonStyle {
        /// 由令牌推导默认样式(androidx 各变体 IconButtonTokens)。
        /// `selected` 为 toggle 选中态。
        pub fn resolve(tokens: &TokenSet, variant: IconButtonVariant, selected: bool) -> Self {
            Self::resolve_with_size(
                tokens,
                variant,
                selected,
                IconButtonSize::Small,
                IconButtonShape::Round,
            )
        }

        /// 按变体、切换态、尺寸与形状解析样式。
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
