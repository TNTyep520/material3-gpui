//! 导航组件样式（对应 m3fx `navigation-bar.css` / `navigation-rail.css` /
//! `navigation-drawer.css` / `top-app-bar.css`）。

use gpui::{Hsla, Pixels, px};

use crate::theme::TokenSet;

/// 导航项（NavigationBar / Rail / Drawer 共用）样式。
#[derive(Clone, Copy, Debug)]
pub struct NavigationItemStyle {
    /// 指示条胶囊宽/高。
    pub indicator_size: (Pixels, Pixels),
    /// 指示条圆角。
    pub indicator_radius: Pixels,
    /// 选中指示条色。
    pub indicator_color: Hsla,
    /// 选中项图标色。
    pub selected_icon_color: Hsla,
    /// 未选中项图标色。
    pub unselected_icon_color: Hsla,
    /// 选中项标签色。
    pub selected_label_color: Hsla,
    /// 未选中项标签色。
    pub unselected_label_color: Hsla,
    /// 图标尺寸。
    pub icon_size: Pixels,
    /// 标签字型。
    pub label: crate::theme::TypeStyle,
    /// hover 状态层不透明度。
    pub hover_opacity: f32,
    /// 按压状态层不透明度。
    pub pressed_opacity: f32,
}

impl NavigationItemStyle {
    /// 由令牌推导默认样式。
    pub fn resolve(tokens: &TokenSet) -> Self {
        let colors = &tokens.colors;
        Self {
            indicator_size: (px(64.), px(32.)),
            indicator_radius: tokens.shapes.full,
            indicator_color: colors.secondary_container,
            selected_icon_color: colors.on_surface_variant,
            unselected_icon_color: colors.on_surface_variant,
            selected_label_color: colors.on_surface,
            unselected_label_color: colors.on_surface_variant,
            icon_size: px(24.),
            label: tokens.typography.label_medium,
            hover_opacity: crate::theme::HOVER_OPACITY,
            pressed_opacity: crate::theme::PRESSED_OPACITY,
        }
    }
}

/// NavigationBar 样式。
#[derive(Clone, Copy, Debug)]
pub struct NavigationBarStyle {
    /// 容器色。
    pub container_color: Hsla,
    /// 高度。
    pub height: Pixels,
    /// 项间距（项内部 图标区/标签 间距）。
    pub item_gap: Pixels,
    /// 项内边距。
    pub item_padding: Pixels,
}

impl NavigationBarStyle {
    /// 由令牌推导默认样式。
    pub fn resolve(tokens: &TokenSet) -> Self {
        Self {
            container_color: tokens.colors.surface_container,
            height: px(80.),
            item_gap: px(4.),
            item_padding: px(8.),
        }
    }
}

/// NavigationRail 样式。
#[derive(Clone, Copy, Debug)]
pub struct NavigationRailStyle {
    /// 容器色。
    pub container_color: Hsla,
    /// 宽度。
    pub width: Pixels,
    /// 项间距。
    pub item_gap: Pixels,
    /// 顶部内边距。
    pub top_padding: Pixels,
}

impl NavigationRailStyle {
    /// 由令牌推导默认样式。
    pub fn resolve(tokens: &TokenSet) -> Self {
        Self {
            container_color: tokens.colors.surface_container,
            width: px(80.),
            item_gap: px(4.),
            top_padding: px(8.),
        }
    }
}

/// NavigationDrawer 样式。
#[derive(Clone, Copy, Debug)]
pub struct NavigationDrawerStyle {
    /// 容器色。
    pub container_color: Hsla,
    /// 标准宽度。
    pub width: Pixels,
    /// modal 宽度。
    pub modal_width: Pixels,
    /// 分组标题色。
    pub section_header_color: Hsla,
    /// 内边距。
    pub padding: Pixels,
    /// 项水平内边距。
    pub item_horizontal_padding: Pixels,
    /// 阴影颜色。
    pub shadow_color: Hsla,
    /// modal 阴影等级。
    pub modal_elevation: crate::theme::Elevation,
    /// 分组标题字型。
    pub section_header: crate::theme::TypeStyle,
}

impl NavigationDrawerStyle {
    /// 由令牌推导默认样式。
    pub fn resolve(tokens: &TokenSet, _modal: bool) -> Self {
        Self {
            container_color: tokens.colors.surface_container_low,
            width: px(360.),
            modal_width: px(360.),
            section_header_color: tokens.colors.on_surface_variant,
            padding: px(12.),
            item_horizontal_padding: px(12.),
            shadow_color: tokens.colors.shadow,
            modal_elevation: crate::theme::Elevation::Level1,
            section_header: tokens.typography.title_small,
        }
    }

    /// 当前宽度。
    pub fn drawer_width(&self, modal: bool) -> Pixels {
        if modal { self.modal_width } else { self.width }
    }
}

/// TopAppBar 样式。
#[derive(Clone, Copy, Debug)]
pub struct TopAppBarStyle {
    /// 容器色。
    pub container_color: Hsla,
    /// 标题色。
    pub title_color: Hsla,
    /// 图标色。
    pub icon_color: Hsla,
    /// 高度。
    pub height: Pixels,
    /// 水平内边距。
    pub horizontal_padding: Pixels,
    /// 元素间距。
    pub gap: Pixels,
    /// 标题字型。
    pub title: crate::theme::TypeStyle,
}

impl TopAppBarStyle {
    /// 由令牌推导默认样式。
    pub fn resolve(tokens: &TokenSet) -> Self {
        Self {
            container_color: tokens.colors.surface,
            title_color: tokens.colors.on_surface,
            icon_color: tokens.colors.on_surface_variant,
            height: px(64.),
            horizontal_padding: px(16.),
            gap: px(8.),
            title: tokens.typography.title_large,
        }
    }
}
