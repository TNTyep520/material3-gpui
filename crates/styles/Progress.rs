//! 进度指示器样式（对应 m3fx `styles/controls/progress.css`）。

use gpui::{Hsla, Pixels, px};

use crate::theme::TokenSet;

/// 线性进度条样式。
#[derive(Clone, Copy, Debug)]
pub struct LinearProgressStyle {
    /// 活动条颜色。
    pub active_color: Hsla,
    /// 轨道颜色。
    pub track_color: Hsla,
    /// 高度。
    pub height: Pixels,
    /// 圆角。
    pub corner_radius: Pixels,
}

impl LinearProgressStyle {
    /// 由令牌推导默认样式。
    pub fn resolve(tokens: &TokenSet) -> Self {
        let colors = &tokens.colors;
        Self {
            active_color: colors.primary,
            track_color: colors.surface_container_highest,
            height: px(4.),
            corner_radius: tokens.shapes.full,
        }
    }
}

/// 环形进度指示器样式。
#[derive(Clone, Copy, Debug)]
pub struct CircularProgressStyle {
    /// 颜色。
    pub color: Hsla,
    /// 默认尺寸。
    pub size: Pixels,
}

impl CircularProgressStyle {
    /// 由令牌推导默认样式。
    pub fn resolve(tokens: &TokenSet) -> Self {
        Self {
            color: tokens.colors.primary,
            size: px(48.),
        }
    }
}
