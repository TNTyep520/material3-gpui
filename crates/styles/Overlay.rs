//! 弹层组件样式（对应 m3fx `snackbar.css` / `menu.css` / `tooltip.css`）。

use gpui::{Hsla, Pixels, px};

use crate::theme::TokenSet;

/// Snackbar 样式。
#[derive(Clone, Copy, Debug)]
pub struct SnackbarStyle {
    /// 容器色。
    pub container_color: Hsla,
    /// 文本色。
    pub text_color: Hsla,
    /// 动作文本色。
    pub action_color: Hsla,
    /// 最小高度。
    pub min_height: Pixels,
    /// 水平/垂直内边距。
    pub padding: (Pixels, Pixels),
    /// 与窗口底边间距。
    pub bottom_offset: Pixels,
    /// 动作与消息间距。
    pub action_gap: Pixels,
    /// 圆角。
    pub corner_radius: Pixels,
    /// 宽度。
    pub width: Pixels,
    /// 阴影颜色。
    pub shadow_color: Hsla,
    /// 消息字型。
    pub text: crate::theme::TypeStyle,
    /// 动作字型。
    pub action: crate::theme::TypeStyle,
}

impl SnackbarStyle {
    /// 由令牌推导默认样式。
    pub fn resolve(tokens: &TokenSet) -> Self {
        let colors = &tokens.colors;
        let tokens_sb = &tokens.component.snackbar;
        Self {
            container_color: colors.inverse_surface,
            text_color: colors.inverse_on_surface,
            action_color: colors.primary,
            min_height: px(tokens_sb.min_height),
            padding: (
                px(tokens_sb.horizontal_padding),
                px(tokens_sb.vertical_padding),
            ),
            bottom_offset: px(tokens_sb.bottom_offset),
            action_gap: px(tokens_sb.action_gap),
            corner_radius: tokens.shapes.extra_small,
            width: px(440.),
            shadow_color: colors.shadow,
            text: tokens.typography.body_medium,
            action: tokens.typography.label_large,
        }
    }
}

/// 菜单样式。
#[derive(Clone, Copy, Debug)]
pub struct MenuStyle {
    /// 容器色。
    pub container_color: Hsla,
    /// 菜单项 hover 状态层不透明度。
    pub item_hover_opacity: Hsla,
    /// 菜单项文本色。
    pub item_text_color: Hsla,
    /// 菜单图标色。
    pub item_icon_color: Hsla,
    /// 菜单项高度。
    pub item_height: Pixels,
    /// 容器垂直内边距。
    pub vertical_padding: Pixels,
    /// 菜单项水平内边距。
    pub item_horizontal_padding: Pixels,
    /// 元素间距。
    pub item_gap: Pixels,
    /// 最小宽度。
    pub min_width: Pixels,
    /// 圆角。
    pub corner_radius: Pixels,
    /// 与锚点间距。
    pub anchor_gap: Pixels,
    /// 阴影颜色。
    pub shadow_color: Hsla,
    /// 菜单项字型。
    pub item_text: crate::theme::TypeStyle,
}

impl MenuStyle {
    /// 由令牌推导默认样式。
    pub fn resolve(tokens: &TokenSet) -> Self {
        let colors = &tokens.colors;
        let menu = &tokens.component.menu;
        Self {
            container_color: colors.surface_container,
            item_hover_opacity: colors.on_surface.opacity(0.08),
            item_text_color: colors.on_surface,
            item_icon_color: colors.on_surface_variant,
            item_height: px(menu.item_height),
            vertical_padding: px(menu.vertical_padding),
            item_horizontal_padding: px(menu.item_horizontal_padding),
            item_gap: px(12.),
            min_width: px(180.),
            corner_radius: px(menu.corner_radius),
            anchor_gap: px(menu.anchor_gap),
            shadow_color: colors.shadow,
            item_text: tokens.typography.label_large,
        }
    }
}

/// Tooltip 样式。
#[derive(Clone, Copy, Debug)]
pub struct TooltipStyle {
    /// 容器色。
    pub container_color: Hsla,
    /// 文本色。
    pub text_color: Hsla,
    /// 高度。
    pub height: Pixels,
    /// 水平内边距。
    pub horizontal_padding: Pixels,
    /// 与锚点间距。
    pub anchor_gap: Pixels,
    /// 圆角。
    pub corner_radius: Pixels,
    /// 文字字型。
    pub text: crate::theme::TypeStyle,
}

impl TooltipStyle {
    /// 由令牌推导默认样式。
    pub fn resolve(tokens: &TokenSet) -> Self {
        let colors = &tokens.colors;
        let tooltip = &tokens.component.tooltip;
        Self {
            container_color: colors.inverse_surface,
            text_color: colors.inverse_on_surface,
            height: px(tooltip.height),
            horizontal_padding: px(tooltip.horizontal_padding),
            anchor_gap: px(6.),
            corner_radius: tokens.shapes.extra_small,
            text: tokens.typography.body_small,
        }
    }
}
