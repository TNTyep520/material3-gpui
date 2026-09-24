//! MD3 令牌 Profile。
//!
//! 移植自 [m3fx](https://github.com/Glavo/m3fx) 的
//! `org.glavo.m3fx.tokens.M3Profile`（Apache-2.0，© 2026 Glavo）。
//!
//! Profile 是一份"预设身份"，决定动态色的规格版本、色彩风格，
//! 以及 typography / shape / motion / 组件令牌的默认家族。
//!
//! 当前仅提供 2021 基线（Material Design 3 baseline）。

#[cfg(feature = "dynamic-color")]
use mcu_dynamiccolor::{SpecVersion, Variant};

/// 令牌 Profile 预设。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Profile {
    /// 2021 基线（Material Design 3 baseline）：SPEC_2021 + TONAL_SPOT。
    #[default]
    Baseline2021,
}

impl Profile {
    /// 该 Profile 使用的动态色规格版本。
    #[cfg(feature = "dynamic-color")]
    pub fn color_spec_version(self) -> SpecVersion {
        match self {
            Profile::Baseline2021 => SpecVersion::Spec2021,
        }
    }

    /// 该 Profile 使用的色彩风格（动态色 Variant）。
    #[cfg(feature = "dynamic-color")]
    pub fn color_style(self) -> Variant {
        match self {
            Profile::Baseline2021 => Variant::TonalSpot,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn profile_is_baseline_2021() {
        // color_spec_version/color_style 依赖 mcu 类型,在
        // dynamic-color feature 的测试中覆盖
        assert!(matches!(
            super::Profile::Baseline2021,
            super::Profile::Baseline2021
        ));
    }
}
