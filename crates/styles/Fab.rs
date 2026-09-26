//! FAB 样式（对应 m3fx `styles/controls/floating-action-button.css`）。

use gpui::{Pixels, px};

use crate::theme::{Elevation, TokenSet};

/// FAB 尺寸档位。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FabSize {
    /// 40dp。
    Small,
    /// 56dp（默认）。
    #[default]
    Standard,
    /// 96dp。
    Large,
}

/// FAB 配色档位。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FabColor {
    /// 表面。
    Surface,
    /// 主色（默认）。
    #[default]
    Primary,
    /// 次级。
    Secondary,
    /// 第三。
    Tertiary,
}

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
            FabSize::Large => (px(96.), shapes.extra_large, px(36.)),
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
