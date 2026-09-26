//! MD3 交互状态（State Layer）令牌与颜色混合工具。
//!
//! 对应 [m3fx](https://github.com/Glavo/m3fx) 的 `M3StateLayerTokens`
//! （Apache-2.0，© 2026 Glavo）。数值与 m3fx 一致：
//! hover 8%、focus/pressed 10%、dragged 16%。
//! 参考: <https://m3.material.io/foundations/interaction/states>

use gpui::{Hsla, Pixels, Rgba, px};

/// hover 状态层不透明度（8%）。
pub const HOVER_OPACITY: f32 = 0.08;
/// focus 状态层不透明度（10%）。
pub const FOCUS_OPACITY: f32 = 0.10;
/// pressed 状态层不透明度（10%）。
pub const PRESSED_OPACITY: f32 = 0.10;
/// dragged 状态层不透明度（16%）。
pub const DRAGGED_OPACITY: f32 = 0.16;
/// 禁用态容器不透明度（12%）。
pub const DISABLED_CONTAINER_OPACITY: f32 = 0.12;
/// 禁用态内容（文字/图标）不透明度（38%）。
pub const DISABLED_CONTENT_OPACITY: f32 = 0.38;

/// 状态层与焦点指示器令牌。
#[derive(Clone, Copy, Debug)]
pub struct StateLayerTokens {
    /// hover 状态层不透明度。
    pub hover: f32,
    /// focus 状态层不透明度。
    pub focus: f32,
    /// pressed 状态层不透明度。
    pub pressed: f32,
    /// dragged 状态层不透明度。
    pub dragged: f32,
    /// 禁用态容器不透明度。
    pub disabled_container: f32,
    /// 禁用态内容不透明度。
    pub disabled_content: f32,
    /// 焦点指示器粗细。
    pub focus_indicator_thickness: Pixels,
    /// 焦点指示器外偏移（相对组件边缘，正值向外）。
    pub focus_indicator_outer_offset: Pixels,
    /// 焦点指示器内偏移（负值向内）。
    pub focus_indicator_inner_offset: Pixels,
}

impl Default for StateLayerTokens {
    /// 基线状态层令牌。
    fn default() -> Self {
        Self::baseline()
    }
}

impl StateLayerTokens {
    /// 基线（2021）状态层令牌。
    pub fn baseline() -> Self {
        Self {
            hover: HOVER_OPACITY,
            focus: FOCUS_OPACITY,
            pressed: PRESSED_OPACITY,
            dragged: DRAGGED_OPACITY,
            disabled_container: DISABLED_CONTAINER_OPACITY,
            disabled_content: DISABLED_CONTENT_OPACITY,
            focus_indicator_thickness: px(3.0),
            focus_indicator_outer_offset: px(2.0),
            focus_indicator_inner_offset: px(-2.0),
        }
    }
}

/// 把 `overlay` 颜色以 `opacity` 不透明度叠加到不透明的 `base` 上，
/// 返回混合后的实色。用于在实色容器上绘制 state layer。
pub fn blend(base: Hsla, overlay: Hsla, opacity: f32) -> Hsla {
    let b: Rgba = base.into();
    let o: Rgba = overlay.into();
    Rgba {
        r: b.r + (o.r - b.r) * opacity,
        g: b.g + (o.g - b.g) * opacity,
        b: b.b + (o.b - b.b) * opacity,
        a: b.a,
    }
    .into()
}

/// hover 状态层：在实色 `base` 上叠加 8% 的 `content` 色。
pub fn hover_layer(base: Hsla, content: Hsla) -> Hsla {
    blend(base, content, HOVER_OPACITY)
}

/// pressed 状态层：在实色 `base` 上叠加 10% 的 `content` 色。
pub fn pressed_layer(base: Hsla, content: Hsla) -> Hsla {
    blend(base, content, PRESSED_OPACITY)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::rgb;

    #[test]
    fn blend_full_opacity_returns_overlay() {
        let base = rgb(0x000000).into();
        let overlay = rgb(0xffffff).into();
        let mixed = blend(base, overlay, 1.0);
        let rgba: Rgba = mixed.into();
        assert!(rgba.r > 0.99);
    }

    #[test]
    fn baseline_matches_m3fx() {
        let t = StateLayerTokens::baseline();
        assert_eq!(t.hover, 0.08);
        assert_eq!(t.focus, 0.10);
        assert_eq!(t.pressed, 0.10);
        assert_eq!(t.dragged, 0.16);
    }
}
