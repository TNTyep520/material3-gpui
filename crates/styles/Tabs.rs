//! Tabs 样式（对应 m3fx `styles/controls/tab-bar.css`）。

use gpui::{Hsla, Pixels, px};

use crate::theme::TokenSet;

/// MD3 标签栏样式。
#[derive(Clone, Copy, Debug)]
pub struct TabBarStyle {
    /// 栏背景色。
    pub container_color: Hsla,
    /// 底部分隔线色。
    pub divider_color: Hsla,
    /// 选中项内容色。
    pub selected_item_color: Hsla,
    /// 未选中项内容色。
    pub unselected_item_color: Hsla,
    /// 指示条颜色。
    pub indicator_color: Hsla,
    /// 指示条高/宽。
    pub indicator_size: (Pixels, Pixels),
    /// 无图标时栏高。
    pub height: Pixels,
    /// 带图标时栏高。
    pub height_with_icon: Pixels,
    /// 图标尺寸。
    pub icon_size: Pixels,
    /// 图标与文字间距。
    pub gap: Pixels,
    /// hover 状态层不透明度。
    pub hover_opacity: f32,
    /// 按压状态层不透明度。
    pub pressed_opacity: f32,
    /// 标签字型。
    pub label: crate::theme::TypeStyle,
}

impl TabBarStyle {
    /// 由令牌推导默认样式。
    pub fn resolve(tokens: &TokenSet, _has_icons: bool) -> Self {
        let colors = &tokens.colors;
        Self {
            container_color: colors.surface,
            divider_color: colors.outline_variant,
            selected_item_color: colors.primary,
            unselected_item_color: colors.on_surface_variant,
            indicator_color: colors.primary,
            indicator_size: (px(3.), px(48.)),
            height: px(48.),
            height_with_icon: px(64.),
            icon_size: px(24.),
            gap: px(4.),
            hover_opacity: crate::theme::HOVER_OPACITY,
            pressed_opacity: crate::theme::PRESSED_OPACITY,
            label: tokens.typography.title_small,
        }
    }
}

impl TabBarStyle {
    /// 当前使用的栏高。
    pub fn bar_height(&self, has_icons: bool) -> Pixels {
        if has_icons {
            self.height_with_icon
        } else {
            self.height
        }
    }
}
