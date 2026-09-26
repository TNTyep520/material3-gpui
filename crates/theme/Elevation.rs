//! MD3 高度（Elevation）令牌
//!
//! 对应 [m3fx](https://github.com/Glavo/m3fx) 的 `M3ElevationTokens`
//! （Apache-2.0，© 2026 Glavo）；阴影参数取自 MD3 官方 elevation 规范
//! （level 0–5）。
//! 参考: <https://m3.material.io/styles/elevation/tokens>

use gpui::{BoxShadow, Hsla, Pixels, point, px};

/// 高度等级的 dp 值令牌（对应 m3fx `M3ElevationTokens`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ElevationTokens {
    /// level 0 的 dp 值。
    pub level0: Pixels,
    /// level 1 的 dp 值。
    pub level1: Pixels,
    /// level 2 的 dp 值。
    pub level2: Pixels,
    /// level 3 的 dp 值。
    pub level3: Pixels,
    /// level 4 的 dp 值。
    pub level4: Pixels,
    /// level 5 的 dp 值。
    pub level5: Pixels,
}

impl Default for ElevationTokens {
    /// 基线（2021）高度刻度：0 / 1 / 3 / 6 / 8 / 12 dp。
    fn default() -> Self {
        Self::baseline()
    }
}

impl ElevationTokens {
    /// 基线（2021）高度刻度。
    pub fn baseline() -> Self {
        Self {
            level0: px(0.),
            level1: px(1.),
            level2: px(3.),
            level3: px(6.),
            level4: px(8.),
            level5: px(12.),
        }
    }
}

/// 高度等级。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Elevation {
    /// level 0。
    Level0,
    /// level 1。
    Level1,
    /// level 2。
    Level2,
    /// level 3。
    Level3,
    /// level 4。
    Level4,
    /// level 5。
    Level5,
}

impl Elevation {
    /// 生成该 elevation 等级对应的一组 BoxShadow。
    /// `shadow_color` 一般传 `theme.colors.shadow`（黑色）。
    pub fn shadows(self, shadow_color: Hsla) -> Vec<BoxShadow> {
        let key = |y: f32, blur: f32, spread: f32| BoxShadow {
            color: shadow_color.opacity(0.30),
            offset: point(px(0.), px(y)),
            blur_radius: px(blur),
            spread_radius: px(spread),
        };
        let ambient = |y: f32, blur: f32, spread: f32| BoxShadow {
            color: shadow_color.opacity(0.15),
            offset: point(px(0.), px(y)),
            blur_radius: px(blur),
            spread_radius: px(spread),
        };
        match self {
            Elevation::Level0 => vec![],
            Elevation::Level1 => vec![key(1., 2., 0.), ambient(1., 3., 1.)],
            Elevation::Level2 => vec![key(1., 2., 0.), ambient(2., 6., 2.)],
            Elevation::Level3 => vec![key(1., 3., 0.), ambient(4., 8., 3.)],
            Elevation::Level4 => vec![key(2., 3., 0.), ambient(6., 10., 4.)],
            Elevation::Level5 => vec![key(4., 4., 0.), ambient(8., 12., 6.)],
        }
    }

    /// 该等级对应的 dp 值（信息性）
    pub fn dp(self) -> Pixels {
        match self {
            Elevation::Level0 => px(0.),
            Elevation::Level1 => px(1.),
            Elevation::Level2 => px(3.),
            Elevation::Level3 => px(6.),
            Elevation::Level4 => px(8.),
            Elevation::Level5 => px(12.),
        }
    }
}
