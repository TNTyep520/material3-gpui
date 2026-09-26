//! 容器组件样式（对应 m3fx `card.css` / `divider.css` / `list-item.css` / `dialog.css`）。

use gpui::{Hsla, Pixels, px};

use crate::theme::TokenSet;

/// 卡片变体（样式解析输入）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardVariant {
    /// 阴影卡片。
    #[default]
    Elevated,
    /// 实色卡片。
    Filled,
    /// 描边卡片。
    Outlined,
}

/// MD3 卡片样式。
#[derive(Clone, Copy, Debug)]
pub struct CardStyle {
    /// 容器色。
    pub container_color: Hsla,
    /// 内容色。
    pub content_color: Hsla,
    /// 描边色（`Some` 启用 1dp 描边）。
    pub outline_color: Option<Hsla>,
    /// 圆角。
    pub corner_radius: Pixels,
    /// 阴影颜色。
    pub shadow_color: Hsla,
    /// 阴影等级。
    pub elevation: crate::theme::Elevation,
}

impl CardStyle {
    /// 由令牌推导默认样式。
    pub fn resolve(tokens: &TokenSet, variant: CardVariant) -> Self {
        let colors = &tokens.colors;
        let (container, outline, elevation) = match variant {
            CardVariant::Elevated => (
                colors.surface_container_low,
                None,
                crate::theme::Elevation::Level1,
            ),
            CardVariant::Filled => (
                colors.surface_container_highest,
                None,
                crate::theme::Elevation::Level0,
            ),
            CardVariant::Outlined => (
                colors.surface,
                Some(colors.outline_variant),
                crate::theme::Elevation::Level0,
            ),
        };
        Self {
            container_color: container,
            content_color: colors.on_surface,
            outline_color: outline,
            corner_radius: tokens.shapes.medium,
            shadow_color: colors.shadow,
            elevation,
        }
    }
}

/// MD3 分隔线样式。
#[derive(Clone, Copy, Debug)]
pub struct DividerStyle {
    /// 颜色。
    pub color: Hsla,
    /// 厚度。
    pub thickness: Pixels,
    /// inset 缩进。
    pub inset: Pixels,
}

impl DividerStyle {
    /// 由令牌推导默认样式。
    pub fn resolve(tokens: &TokenSet, inset: bool) -> Self {
        Self {
            color: tokens.colors.outline_variant,
            thickness: px(1.),
            inset: if inset { px(16.) } else { px(0.) },
        }
    }
}

/// MD3 列表项样式。
#[derive(Clone, Copy, Debug)]
pub struct ListItemStyle {
    /// 单行高度。
    pub height_single_line: Pixels,
    /// 双行高度。
    pub height_two_line: Pixels,
    /// 内容色。
    pub content_color: Hsla,
    /// 支撑文本色。
    pub supporting_color: Hsla,
    /// 尾随元素色。
    pub trailing_color: Hsla,
    /// 水平内边距。
    pub horizontal_padding: Pixels,
    /// 垂直内边距。
    pub vertical_padding: Pixels,
    /// 元素间距。
    pub gap: Pixels,
    /// 图标尺寸。
    pub icon_size: Pixels,
    /// hover 状态层不透明度。
    pub hover_opacity: f32,
    /// 标题字型。
    pub headline: crate::theme::TypeStyle,
    /// 支撑文本字型。
    pub supporting: crate::theme::TypeStyle,
    /// 尾随字型。
    pub trailing: crate::theme::TypeStyle,
}

impl ListItemStyle {
    /// 由令牌推导默认样式。
    pub fn resolve(tokens: &TokenSet) -> Self {
        let colors = &tokens.colors;
        Self {
            height_single_line: px(56.),
            height_two_line: px(72.),
            content_color: colors.on_surface,
            supporting_color: colors.on_surface_variant,
            trailing_color: colors.on_surface_variant,
            horizontal_padding: px(16.),
            vertical_padding: px(8.),
            gap: px(16.),
            icon_size: px(24.),
            hover_opacity: crate::theme::HOVER_OPACITY,
            headline: tokens.typography.body_large,
            supporting: tokens.typography.body_medium,
            trailing: tokens.typography.label_small,
        }
    }
}

/// MD3 对话框样式。
#[derive(Clone, Copy, Debug)]
pub struct DialogStyle {
    /// 容器色。
    pub container_color: Hsla,
    /// 内容色。
    pub content_color: Hsla,
    /// 辅助文本色。
    pub supporting_color: Hsla,
    /// 图标色。
    pub icon_color: Hsla,
    /// scrim 颜色。
    pub scrim_color: Hsla,
    /// scrim 不透明度。
    pub scrim_opacity: f32,
    /// 圆角。
    pub corner_radius: Pixels,
    /// 最小/最大宽度。
    pub width_range: (Pixels, Pixels),
    /// 内边距。
    pub padding: Pixels,
    /// 元素间距。
    pub gap: Pixels,
    /// 阴影颜色。
    pub shadow_color: Hsla,
    /// 阴影等级。
    pub elevation: crate::theme::Elevation,
    /// 标题字型。
    pub title: crate::theme::TypeStyle,
    /// 正文字型。
    pub body: crate::theme::TypeStyle,
}

impl DialogStyle {
    /// 由令牌推导默认样式。
    pub fn resolve(tokens: &TokenSet) -> Self {
        let colors = &tokens.colors;
        Self {
            container_color: colors.surface_container_high,
            content_color: colors.on_surface,
            supporting_color: colors.on_surface_variant,
            icon_color: colors.secondary,
            scrim_color: colors.scrim,
            scrim_opacity: 0.32,
            corner_radius: tokens.shapes.extra_large,
            width_range: (px(280.), px(560.)),
            padding: px(24.),
            gap: px(16.),
            shadow_color: colors.shadow,
            elevation: crate::theme::Elevation::Level3,
            title: tokens.typography.headline_small,
            body: tokens.typography.body_medium,
        }
    }
}
