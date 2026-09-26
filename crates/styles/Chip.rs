//! Chip 样式（对应 m3fx `styles/controls/chip.css`）。

use gpui::{Hsla, Pixels, px};

use crate::theme::TokenSet;

/// 纸片变体（样式解析输入）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChipVariant {
    /// 辅助。
    #[default]
    Assist,
    /// 过滤。
    Filter,
    /// 输入。
    Input,
    /// 建议。
    Suggestion,
}

/// MD3 纸片样式。
#[derive(Clone, Copy, Debug)]
pub struct ChipStyle {
    /// 容器色（`None` 为透明）。
    pub container_color: Option<Hsla>,
    /// 内容色。
    pub content_color: Hsla,
    /// 前导图标色。
    pub icon_color: Hsla,
    /// 描边色（`Some` 启用 1dp 描边）。
    pub outline_color: Option<Hsla>,
    /// 高度。
    pub height: Pixels,
    /// 圆角。
    pub corner_radius: Pixels,
    /// 带前导/尾随元素一侧的水平内边距。
    pub edge_padding: Pixels,
    /// 无前导/尾随元素一侧的水平内边距。
    pub center_padding: Pixels,
    /// 元素间距。
    pub gap: Pixels,
    /// 前导图标尺寸。
    pub icon_size: Pixels,
    /// 移除按钮尺寸。
    pub close_size: Pixels,
    /// 状态层基色。
    pub state_layer_color: Hsla,
    /// 按压档状态层不透明度。
    pub state_layer_opacity: f32,
    /// 文字字型。
    pub label: crate::theme::TypeStyle,
}

impl ChipStyle {
    /// 由令牌推导默认样式。
    pub fn resolve(
        tokens: &TokenSet,
        variant: ChipVariant,
        selected: bool,
        elevated: bool,
        disabled: bool,
    ) -> Self {
        let colors = &tokens.colors;
        let state = &tokens.state_layer;
        let _ = &tokens.component.button; // 几何令牌预留

        let _ = variant; // 首期四变体共享基线几何；颜色差异仅由 selected/elevated 决定
        let (container, content, icon) = if disabled {
            (
                None,
                colors.disabled_content(state),
                colors.disabled_content(state),
            )
        } else if selected {
            (
                Some(colors.secondary_container),
                colors.on_secondary_container,
                colors.on_secondary_container,
            )
        } else if elevated {
            (
                Some(colors.surface_container_low),
                colors.on_surface,
                colors.primary,
            )
        } else {
            (None, colors.on_surface, colors.primary)
        };

        Self {
            container_color: container,
            content_color: content,
            icon_color: icon,
            outline_color: if container.is_none() && !elevated {
                Some(if disabled {
                    colors.disabled_content(state)
                } else {
                    colors.outline_variant
                })
            } else {
                None
            },
            height: px(32.),
            corner_radius: tokens.shapes.small,
            edge_padding: px(8.),
            center_padding: px(16.),
            gap: px(8.),
            icon_size: px(18.),
            close_size: px(16.),
            state_layer_color: content,
            state_layer_opacity: state.pressed,
            label: tokens.typography.label_large,
        }
    }
}
