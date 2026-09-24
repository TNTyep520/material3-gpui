//! 按钮样式（对应 m3fx `styles/controls/button.css`：
//! `.m3-button-base` 及各变体、尺寸、形状、禁用态段落）。

use gpui::{Hsla, Pixels, px};

use crate::theme::{Elevation, TokenSet};

/// 按钮变体（样式解析输入）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    /// 实心主色。
    #[default]
    Filled,
    /// 描边。
    Outlined,
    /// 纯文字。
    Text,
    /// 带高度。
    Elevated,
    /// 次级色调实心。
    FilledTonal,
}

/// 图标按钮变体（样式解析输入）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IconButtonVariant {
    /// 标准。
    #[default]
    Standard,
    /// 实心。
    Filled,
    /// 次级色调。
    FilledTonal,
    /// 描边。
    Outlined,
}

/// MD3 共享按钮样式。
#[derive(Clone, Debug)]
pub struct ButtonStyle {
    /// 容器色（`None` 为透明容器）。
    pub container_color: Option<Hsla>,
    /// 内容色（文字/图标/状态层基色）。
    pub content_color: Hsla,
    /// 描边色（`Some` 启用 1dp 描边）。
    pub outline_color: Option<Hsla>,
    /// 阴影等级。
    pub elevation: Elevation,
    /// 阴影颜色。
    pub shadow_color: Hsla,
    /// 禁用态容器色。
    pub disabled_container_color: Hsla,
    /// 禁用态内容色。
    pub disabled_content_color: Hsla,
    /// 状态层/涟漪基色。
    pub state_layer_color: Hsla,
    /// 按压档状态层不透明度。
    pub state_layer_opacity: f32,
    /// 容器高度。
    pub height: Pixels,
    /// 圆角。
    pub corner_radius: Pixels,
    /// 水平内边距（左, 右）。
    pub padding: (Pixels, Pixels),
    /// 图标尺寸。
    pub icon_size: Pixels,
    /// 图标与文字间距。
    pub icon_gap: Pixels,
    /// 文字字型。
    pub label: crate::theme::TypeStyle,
}

impl ButtonStyle {
    /// 由令牌推导默认样式。
    ///
    /// `leading_icon`/`trailing_icon` 影响内边距（对应 button.css 的
    /// `:has(...)` 内边距规则）。
    pub fn resolve(
        tokens: &TokenSet,
        variant: ButtonVariant,
        leading_icon: bool,
        trailing_icon: bool,
    ) -> Self {
        Self::resolve_inner(tokens, variant, leading_icon, trailing_icon, false)
    }

    /// 禁用态样式。
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

        // (容器色, 内容色, 描边, elevation) —— 对齐 button.css 变体段落
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

        // 内边距：Text 变体 12dp，带图标侧 16dp，其余 24dp
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
                Some(colors.disabled_container(state))
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
                    // Compose M3：禁用描边 = outline @ 12%
                    colors.outline.opacity(state.disabled_container)
                } else {
                    c
                }
            }),
            elevation,
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
    /// 由令牌推导默认样式。`selected` 为 toggle 选中态。
    pub fn resolve(tokens: &TokenSet, variant: IconButtonVariant, selected: bool) -> Self {
        let colors = &tokens.colors;
        let state = &tokens.state_layer;

        let (container, content, outline) = match (variant, selected) {
            (IconButtonVariant::Standard, false) => (None, colors.on_surface_variant, None),
            (IconButtonVariant::Standard, true) => (None, colors.primary, None),
            (IconButtonVariant::Filled, false) => {
                (Some(colors.surface_container_highest), colors.primary, None)
            }
            (IconButtonVariant::Filled, true) => (Some(colors.primary), colors.on_primary, None),
            (IconButtonVariant::FilledTonal, false) => (
                Some(colors.surface_container_highest),
                colors.on_surface_variant,
                None,
            ),
            (IconButtonVariant::FilledTonal, true) => (
                Some(colors.secondary_container),
                colors.on_secondary_container,
                None,
            ),
            (IconButtonVariant::Outlined, false) => {
                (None, colors.on_surface_variant, Some(colors.outline))
            }
            (IconButtonVariant::Outlined, true) => (
                Some(colors.inverse_surface),
                colors.inverse_on_surface,
                None,
            ),
        };

        Self {
            container_color: container,
            content_color: content,
            outline_color: outline,
            size: px(40.),
            icon_size: px(24.),
            corner_radius: tokens.shapes.full,
            state_layer_color: content,
            state_layer_opacity: state.pressed,
            disabled_content_color: colors.disabled_content(state),
        }
    }
}
