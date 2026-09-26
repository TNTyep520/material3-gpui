//! 文本输入样式（对应 m3fx `styles/controls/text-field.css`）。

use gpui::{Hsla, Pixels, px};

use crate::theme::TokenSet;

/// MD3 文本框样式。
#[derive(Clone, Copy, Debug)]
pub struct TextFieldStyle {
    /// 容器背景色。
    pub container_color: Hsla,
    /// 未聚焦描边色。
    pub border_color: Hsla,
    /// 聚焦/error 强调色。
    pub accent: Hsla,
    /// 浮动标签色（未聚焦）。
    pub label_color: Hsla,
    /// 浮动标签色（聚焦）。
    pub focused_label_color: Hsla,
    /// 输入文字色。
    pub text_color: Hsla,
    /// 占位文字色。
    pub placeholder_color: Hsla,
    /// helper 文本色。
    pub helper_color: Hsla,
    /// error 文本色。
    pub error_color: Hsla,
    /// 前导图标色。
    pub icon_color: Hsla,
    /// 禁用容器色。
    pub disabled_container_color: Hsla,
    /// 最小高度。
    pub min_height: Pixels,
    /// 圆角。
    pub corner_radius: Pixels,
    /// 水平内边距。
    pub horizontal_padding: Pixels,
    /// 垂直内边距。
    pub vertical_padding: Pixels,
    /// helper/error 文本与输入区间距。
    pub supporting_gap: Pixels,
    /// 图标尺寸。
    pub icon_size: Pixels,
    /// 输入文字字型。
    pub text: crate::theme::TypeStyle,
    /// 浮动标签字型。
    pub floating_label: crate::theme::TypeStyle,
    /// 支撑文本字型。
    pub supporting_text: crate::theme::TypeStyle,
}

impl TextFieldStyle {
    /// 由令牌推导默认样式。
    pub fn resolve(tokens: &TokenSet, error: bool, disabled: bool) -> Self {
        let colors = &tokens.colors;
        let state = &tokens.state_layer;
        let field = &tokens.component.text_field;
        let accent = if error { colors.error } else { colors.primary };
        Self {
            container_color: if disabled {
                colors.on_surface.opacity(state.disabled_container / 6.0)
            } else {
                colors.surface
            },
            border_color: if disabled {
                colors.disabled_container(state)
            } else if error {
                colors.error
            } else {
                colors.outline
            },
            accent,
            label_color: colors.on_surface_variant,
            focused_label_color: if error { colors.error } else { colors.primary },
            text_color: colors.on_surface,
            placeholder_color: colors.on_surface_variant.opacity(0.7),
            helper_color: colors.on_surface_variant,
            error_color: colors.error,
            icon_color: colors.on_surface_variant,
            disabled_container_color: colors.on_surface.opacity(state.disabled_container / 6.0),
            min_height: px(field.min_height),
            corner_radius: tokens.shapes.extra_small,
            horizontal_padding: px(field.horizontal_padding),
            vertical_padding: px(field.top_padding),
            supporting_gap: px(field.supporting_gap),
            icon_size: px(field.icon_size),
            text: tokens.typography.body_large,
            floating_label: tokens.typography.label_small,
            supporting_text: tokens.typography.body_small,
        }
    }
}
