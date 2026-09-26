//! MD3 密度令牌。
//!
//! 对应 [m3fx](https://github.com/Glavo/m3fx) 的 `M3Density`
//! （Apache-2.0，© 2026 Glavo）。密度影响布局敏感的组件令牌。
//!
//! 首期仅实现标准密度；其余档位为后续期预留。

/// 组件密度档位。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Density {
    /// 标准密度（默认）。
    #[default]
    Standard,
    /// 舒适密度（更大间距）。
    Comfortable,
    /// 紧凑密度（更小间距）。
    Compact,
}
