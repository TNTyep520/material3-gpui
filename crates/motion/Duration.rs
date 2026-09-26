//! MD3 运动时长令牌。
//!
//! 移植自 [m3fx](https://github.com/Glavo/m3fx) 的
//! `org.glavo.m3fx.animation.M3Motion`（Apache-2.0，© 2026 Glavo）。
//! 与 Compose Material 3 的 `MotionDurationTokens` 一致。

use std::time::Duration;

use crate::tokens::MotionTokens;

/// 以毫秒构造运动时长。
pub const fn ms(millis: u64) -> Duration {
    Duration::from_millis(millis)
}

/// SHORT1：50ms。
pub const SHORT1: Duration = MotionTokens::SHORT1;
/// SHORT2：100ms。
pub const SHORT2: Duration = MotionTokens::SHORT2;
/// SHORT3：150ms。
pub const SHORT3: Duration = MotionTokens::SHORT3;
/// SHORT4：200ms。
pub const SHORT4: Duration = MotionTokens::SHORT4;
/// MEDIUM1：250ms。
pub const MEDIUM1: Duration = MotionTokens::MEDIUM1;
/// MEDIUM2：300ms。
pub const MEDIUM2: Duration = MotionTokens::MEDIUM2;
/// MEDIUM3：350ms。
pub const MEDIUM3: Duration = MotionTokens::MEDIUM3;
/// MEDIUM4：400ms。
pub const MEDIUM4: Duration = MotionTokens::MEDIUM4;
/// LONG1：450ms。
pub const LONG1: Duration = MotionTokens::LONG1;
/// LONG2：500ms。
pub const LONG2: Duration = MotionTokens::LONG2;
/// LONG3：550ms。
pub const LONG3: Duration = MotionTokens::LONG3;
/// LONG4：600ms。
pub const LONG4: Duration = MotionTokens::LONG4;
/// EXTRA_LONG1：700ms。
pub const EXTRA_LONG1: Duration = MotionTokens::EXTRA_LONG1;
/// EXTRA_LONG2：800ms。
pub const EXTRA_LONG2: Duration = MotionTokens::EXTRA_LONG2;
/// EXTRA_LONG3：900ms。
pub const EXTRA_LONG3: Duration = MotionTokens::EXTRA_LONG3;
/// EXTRA_LONG4：1000ms。
pub const EXTRA_LONG4: Duration = MotionTokens::EXTRA_LONG4;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_match_spec() {
        assert_eq!(SHORT3, Duration::from_millis(150));
        assert_eq!(LONG2, Duration::from_millis(500));
        assert_eq!(EXTRA_LONG4, Duration::from_millis(1000));
    }
}
