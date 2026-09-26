//! MD3 形状（Shape）令牌：圆角刻度。
//!
//! 对应 [m3fx](https://github.com/Glavo/m3fx) 的 `M3ShapeTokens`
//! （Apache-2.0，© 2026 Glavo），数值与 Compose Material 3 的
//! shape token 一致；同时兼容 material-web 的 `md-sys-shape`。
//! 参考: <https://m3.material.io/styles/shape/shape-scale-tokens>

use gpui::{Pixels, px};

/// MD3 圆角刻度（10 档）。
#[derive(Clone, Copy, Debug)]
pub struct Shapes {
    /// 无圆角。
    pub none: Pixels,
    /// extra-small。
    pub extra_small: Pixels,
    /// small。
    pub small: Pixels,
    /// medium。
    pub medium: Pixels,
    /// large。
    pub large: Pixels,
    /// large-increased（Expressive 新增档）。
    pub large_increased: Pixels,
    /// extra-large。
    pub extra_large: Pixels,
    /// extra-large-increased（Expressive 新增档）。
    pub extra_large_increased: Pixels,
    /// extra-extra-large（Expressive 新增档）。
    pub extra_extra_large: Pixels,
    /// full：胶囊形。以一个大于任何组件尺寸的值表示，
    /// 渲染时通常直接用 gpui 的 `.rounded_full()`。
    pub full: Pixels,
}

impl Default for Shapes {
    /// 基线（2021）圆角刻度。
    fn default() -> Self {
        Self::baseline()
    }
}

impl Shapes {
    /// 基线（2021）圆角刻度。
    pub fn baseline() -> Self {
        Self {
            none: px(0.),
            extra_small: px(4.),
            small: px(8.),
            medium: px(12.),
            large: px(16.),
            large_increased: px(20.),
            extra_large: px(28.),
            extra_large_increased: px(32.),
            extra_extra_large: px(48.),
            full: px(999.),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scales_match_m3fx() {
        let b = Shapes::baseline();
        assert_eq!(b.extra_small, px(4.));
        assert_eq!(b.large_increased, px(20.));
        assert_eq!(b.extra_extra_large, px(48.));
    }
}
