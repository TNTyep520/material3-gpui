//! MD3 运动方案（Motion Scheme）。
//!
//! 移植自 [m3fx](https://github.com/Glavo/m3fx) 的
//! `M3MotionScheme` / `M3MotionSpec` / `M3SpringParameters`（Apache-2.0，© 2026 Glavo）。
//! 按语义角色分组运动规格：effects 角色用于非空间过渡（透明度、颜色），
//! spatial 角色用于位置/尺寸/形状变化；每族各有 fast / default / slow 三档。

use std::time::Duration;

use super::duration as dur;
use super::easing::{self, Easing};
use super::spring::SpringParameters;

/// 运动语义角色。
///
/// effects 角色用于不改变空间位置的过渡（透明度、颜色等）；
/// spatial 角色用于位置、尺寸、形状变化。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MotionRole {
    /// 快速效果。
    FastEffects,
    /// 默认效果。
    DefaultEffects,
    /// 慢速效果。
    SlowEffects,
    /// 快速空间。
    FastSpatial,
    /// 默认空间。
    DefaultSpatial,
    /// 慢速空间。
    SlowSpatial,
}

/// 单个角色的运动规格：弹簧参数 + 弹簧不可用时的回退时长/缓动。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MotionSpec {
    /// 弹簧物理参数。
    pub spring: SpringParameters,
    /// 回退时长（弹簧时长估算失败或系统要求时长动画时使用）。
    pub fallback_duration: Duration,
    /// 回退缓动曲线（与回退时长配对）。
    pub fallback_easing: Easing,
}

impl MotionSpec {
    /// 由弹簧参数与回退值构造规格。
    pub const fn spring(
        spring: SpringParameters,
        fallback_duration: Duration,
        fallback_easing: Easing,
    ) -> Self {
        Self {
            spring,
            fallback_duration,
            fallback_easing,
        }
    }
}

/// MD3 运动方案：按语义角色归组的不可变运动规格集合。
#[derive(Clone, Copy, Debug)]
pub struct MotionScheme {
    specs: [MotionSpec; 6],
}

impl MotionScheme {
    /// 标准（baseline）运动方案。
    pub fn standard() -> Self {
        Self {
            specs: [
                // fastEffects
                MotionSpec::spring(
                    SpringParameters::new(1.0, 3800.0),
                    dur::SHORT3,
                    easing::FAST_EFFECTS,
                ),
                // defaultEffects
                MotionSpec::spring(
                    SpringParameters::new(1.0, 1600.0),
                    dur::SHORT4,
                    easing::DEFAULT_EFFECTS,
                ),
                // slowEffects
                MotionSpec::spring(
                    SpringParameters::new(1.0, 800.0),
                    dur::MEDIUM2,
                    easing::SLOW_EFFECTS,
                ),
                // fastSpatial
                MotionSpec::spring(
                    SpringParameters::new(0.9, 1400.0),
                    dur::MEDIUM3,
                    easing::STANDARD_SPATIAL,
                ),
                // defaultSpatial
                MotionSpec::spring(
                    SpringParameters::new(0.9, 700.0),
                    dur::LONG2,
                    easing::STANDARD_SPATIAL,
                ),
                // slowSpatial
                MotionSpec::spring(
                    SpringParameters::new(0.9, 300.0),
                    Duration::from_millis(750),
                    easing::STANDARD_SPATIAL,
                ),
            ],
        }
    }

    /// 读取某角色的运动规格。
    pub fn spec(&self, role: MotionRole) -> &MotionSpec {
        let index = match role {
            MotionRole::FastEffects => 0,
            MotionRole::DefaultEffects => 1,
            MotionRole::SlowEffects => 2,
            MotionRole::FastSpatial => 3,
            MotionRole::DefaultSpatial => 4,
            MotionRole::SlowSpatial => 5,
        };
        &self.specs[index]
    }

    /// 返回以当前方案为基础的构建器。
    pub fn builder() -> MotionSchemeBuilder {
        MotionSchemeBuilder::new(Self::standard())
    }

    /// 从既有方案派生构建器。
    pub fn rebuild(self) -> MotionSchemeBuilder {
        MotionSchemeBuilder::new(self)
    }
}

/// `MotionScheme` 构建器：可逐角色替换运动规格。
#[derive(Clone, Debug)]
pub struct MotionSchemeBuilder {
    scheme: MotionScheme,
}

impl MotionSchemeBuilder {
    /// 以 `scheme` 为初值创建构建器。
    fn new(scheme: MotionScheme) -> Self {
        Self { scheme }
    }

    /// 覆盖某角色的运动规格。
    pub fn role(mut self, role: MotionRole, spec: MotionSpec) -> Self {
        let index = match role {
            MotionRole::FastEffects => 0,
            MotionRole::DefaultEffects => 1,
            MotionRole::SlowEffects => 2,
            MotionRole::FastSpatial => 3,
            MotionRole::DefaultSpatial => 4,
            MotionRole::SlowSpatial => 5,
        };
        self.scheme.specs[index] = spec;
        self
    }

    /// 构建不可变方案。
    pub fn build(self) -> MotionScheme {
        self.scheme
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_roles_match_spec() {
        let scheme = MotionScheme::standard();
        let fast = scheme.spec(MotionRole::FastEffects);
        assert_eq!(fast.spring, SpringParameters::new(1.0, 3800.0));
        assert_eq!(fast.fallback_duration, dur::SHORT3);
        let slow_spatial = scheme.spec(MotionRole::SlowSpatial);
        assert_eq!(slow_spatial.spring, SpringParameters::new(0.9, 300.0));
        assert_eq!(slow_spatial.fallback_duration, Duration::from_millis(750));
    }

    #[test]
    fn builder_overrides_role() {
        let scheme = MotionScheme::builder()
            .role(
                MotionRole::FastEffects,
                MotionSpec::spring(
                    SpringParameters::new(0.5, 100.0),
                    dur::SHORT1,
                    easing::LINEAR,
                ),
            )
            .build();
        assert_eq!(scheme.spec(MotionRole::FastEffects).spring.stiffness, 100.0);
    }
}
