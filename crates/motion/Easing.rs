//! MD3 运动缓动曲线。
//!
//! 移植自 [m3fx](https://github.com/Glavo/m3fx) 的
//! `org.glavo.m3fx.animation.M3Motion`（Apache-2.0，© 2026 Glavo）。
//! 曲线数值与 Compose Material 3 的 `Easing` 定义一致。
//!
//! 所有曲线把归一化时间 `t ∈ [0, 1]` 映射为归一化进度 `[0, 1]`。

use std::time::Duration;

/// 缓动曲线。
///
/// `CubicBezier` 覆盖 MD3 的全部标准曲线；`Emphasized` 是
/// 两段三次贝塞尔拼接的三点曲线（中点 `(1/6, 0.4)`）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Easing {
    /// 线性。
    Linear,
    /// 三次贝塞尔曲线，参数为两个控制点 `(x1, y1, x2, y2)`，
    /// 起点 `(0, 0)`、终点 `(1, 1)`，控制点 x 必须位于 `(0, 1)` 内。
    CubicBezier {
        /// 控制点 1 的 x
        x1: f64,
        /// 控制点 1 的 y
        y1: f64,
        /// 控制点 2 的 x
        x2: f64,
        /// 控制点 2 的 y
        y2: f64,
    },
    /// 两段三次贝塞尔拼接曲线，中点 `(1/6, 0.4)`。
    Emphasized,
}

impl Easing {
    /// 采样曲线在归一化时间 `t` 处的进度值。
    ///
    /// `t` 会被夹取到 `[0, 1]`。
    pub fn sample(&self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        match *self {
            Easing::Linear => t,
            Easing::CubicBezier { x1, y1, x2, y2 } => unit_cubic_y(t, x1, y1, x2, y2),
            Easing::Emphasized => {
                const MID_X: f64 = 0.166_666;
                const MID_Y: f64 = 0.4;
                if t < MID_X {
                    // 前段：(0,0) -> (MID_X, MID_Y)，控制点 (0.05, 0.0)、(0.133333, 0.06)
                    segment_y(t, MID_X, MID_Y, 0.0, 0.0, 0.05, 0.0, 0.133_333, 0.06)
                } else {
                    // 后段：(MID_X, MID_Y) -> (1, 1)，控制点 (0.208333, 0.82)、(0.25, 1.0)
                    segment_y(t, 1.0, 1.0, MID_X, MID_Y, 0.208_333, 0.82, 0.25, 1.0)
                }
            }
        }
    }

    /// 按时长采样：把已经过时间映射为归一化时间后再求进度。
    pub fn sample_duration(&self, elapsed: Duration, duration: Duration) -> f64 {
        if duration.is_zero() {
            return 1.0;
        }
        let t = elapsed.as_secs_f64() / duration.as_secs_f64();
        self.sample(t)
    }
}

/// 解单位三次贝塞尔（起点 `(0,0)`、终点 `(1,1)`）在 `x = t` 处的 `y` 值。
///
/// 算法与 m3fx `M3Motion.cubicBezier` 一致：二分法（24 次迭代）
/// 求解 x 多项式的参数，再代入 y 多项式求值。
fn unit_cubic_y(t: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    if t <= 0.0 {
        return 0.0;
    }
    if t >= 1.0 {
        return 1.0;
    }
    let ax = 1.0 - 3.0 * x2 + 3.0 * x1;
    let bx = 3.0 * x2 - 6.0 * x1;
    let cx = 3.0 * x1;
    let ay = 1.0 - 3.0 * y2 + 3.0 * y1;
    let by = 3.0 * y2 - 6.0 * y1;
    let cy = 3.0 * y1;

    // x(s) 关于参数 s 单调（控制点 x 位于 (0,1) 内），二分求 s
    let mut lo = 0.0;
    let mut hi = 1.0;
    let mut s = t;
    for _ in 0..24 {
        s = (lo + hi) / 2.0;
        let x = ((ax * s + bx) * s + cx) * s;
        if (x - t).abs() < 1e-9 {
            break;
        }
        if x < t {
            lo = s;
        } else {
            hi = s;
        }
    }
    ((ay * s + by) * s + cy) * s
}

/// 求一般三次贝塞尔段在横坐标 `t` 处的纵坐标。
///
/// 参数为段终点 `(to_x, to_y)`、段起点 `(from_x, from_y)`
/// 与两个控制点；数学上需要 9 个参数。
#[allow(clippy::too_many_arguments)]
fn segment_y(
    t: f64,
    to_x: f64,
    to_y: f64,
    from_x: f64,
    from_y: f64,
    c1x: f64,
    c1y: f64,
    c2x: f64,
    c2y: f64,
) -> f64 {
    let span_x = to_x - from_x;
    let span_y = to_y - from_y;
    if t <= from_x {
        return from_y;
    }
    if t >= to_x {
        return to_y;
    }
    // 把段内归一化参数 s 与全局 t 关联：x(s) = from_x + span_x * u(s)，
    // 其中 u(s) 是以 (0,0)->(1,1)、控制点按段宽归一化的单位贝塞尔。
    let u1x = (c1x - from_x) / span_x;
    let u2x = (c2x - from_x) / span_x;
    let u1y = if span_y != 0.0 {
        (c1y - from_y) / span_y
    } else {
        0.0
    };
    let u2y = if span_y != 0.0 {
        (c2y - from_y) / span_y
    } else {
        0.0
    };
    let local_t = (t - from_x) / span_x;
    from_y + unit_cubic_y(local_t, u1x, u1y, u2x, u2y) * span_y
}

/// 线性曲线常量。
pub const LINEAR: Easing = Easing::Linear;
/// 标准（standard）曲线：`cubic-bezier(0.2, 0, 0, 1)`。
pub const STANDARD: Easing = Easing::CubicBezier {
    x1: 0.2,
    y1: 0.0,
    x2: 0.0,
    y2: 1.0,
};
/// 标准加速曲线：`cubic-bezier(0.3, 0, 1, 1)`。
pub const STANDARD_ACCELERATE: Easing = Easing::CubicBezier {
    x1: 0.3,
    y1: 0.0,
    x2: 1.0,
    y2: 1.0,
};
/// 标准减速曲线：`cubic-bezier(0, 0, 0, 1)`。
pub const STANDARD_DECELERATE: Easing = Easing::CubicBezier {
    x1: 0.0,
    y1: 0.0,
    x2: 0.0,
    y2: 1.0,
};
/// 强调加速曲线：`cubic-bezier(0.3, 0, 0.8, 0.15)`。
pub const EMPHASIZED_ACCELERATE: Easing = Easing::CubicBezier {
    x1: 0.3,
    y1: 0.0,
    x2: 0.8,
    y2: 0.15,
};
/// 强调减速曲线：`cubic-bezier(0.05, 0.7, 0.1, 1)`。
pub const EMPHASIZED_DECELERATE: Easing = Easing::CubicBezier {
    x1: 0.05,
    y1: 0.7,
    x2: 0.1,
    y2: 1.0,
};
/// 标准空间曲线（spring 的有限回退近似）：`cubic-bezier(0.27, 1.06, 0.18, 1)`。
pub const STANDARD_SPATIAL: Easing = Easing::CubicBezier {
    x1: 0.27,
    y1: 1.06,
    x2: 0.18,
    y2: 1.0,
};
/// Expressive 快速空间曲线：`cubic-bezier(0.42, 1.67, 0.21, 0.9)`。
pub const EXPRESSIVE_FAST_SPATIAL: Easing = Easing::CubicBezier {
    x1: 0.42,
    y1: 1.67,
    x2: 0.21,
    y2: 0.9,
};
/// Expressive 默认空间曲线：`cubic-bezier(0.38, 1.21, 0.22, 1)`。
pub const EXPRESSIVE_DEFAULT_SPATIAL: Easing = Easing::CubicBezier {
    x1: 0.38,
    y1: 1.21,
    x2: 0.22,
    y2: 1.0,
};
/// Expressive 慢速空间曲线：`cubic-bezier(0.39, 1.29, 0.35, 0.98)`。
pub const EXPRESSIVE_SLOW_SPATIAL: Easing = Easing::CubicBezier {
    x1: 0.39,
    y1: 1.29,
    x2: 0.35,
    y2: 0.98,
};
/// 快速效果曲线：`cubic-bezier(0.31, 0.94, 0.34, 1)`。
pub const FAST_EFFECTS: Easing = Easing::CubicBezier {
    x1: 0.31,
    y1: 0.94,
    x2: 0.34,
    y2: 1.0,
};
/// 默认效果曲线：`cubic-bezier(0.34, 0.8, 0.34, 1)`。
pub const DEFAULT_EFFECTS: Easing = Easing::CubicBezier {
    x1: 0.34,
    y1: 0.8,
    x2: 0.34,
    y2: 1.0,
};
/// 慢速效果曲线：`cubic-bezier(0.34, 0.88, 0.34, 1)`。
pub const SLOW_EFFECTS: Easing = Easing::CubicBezier {
    x1: 0.34,
    y1: 0.88,
    x2: 0.34,
    y2: 1.0,
};
/// 强调曲线（三段式）：`Easing::Emphasized`。
pub const EMPHASIZED: Easing = Easing::Emphasized;

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn endpoints_are_identity() {
        for easing in [
            STANDARD,
            STANDARD_ACCELERATE,
            STANDARD_DECELERATE,
            EMPHASIZED_ACCELERATE,
            EMPHASIZED_DECELERATE,
            STANDARD_SPATIAL,
            EXPRESSIVE_FAST_SPATIAL,
            EXPRESSIVE_DEFAULT_SPATIAL,
            EXPRESSIVE_SLOW_SPATIAL,
            FAST_EFFECTS,
            DEFAULT_EFFECTS,
            SLOW_EFFECTS,
            EMPHASIZED,
        ] {
            assert!(close(easing.sample(0.0), 0.0), "{easing:?} at 0");
            assert!(close(easing.sample(1.0), 1.0), "{easing:?} at 1");
        }
    }

    #[test]
    fn linear_is_identity() {
        assert!(close(Easing::Linear.sample(0.25), 0.25));
        assert!(close(Easing::Linear.sample(0.75), 0.75));
    }

    #[test]
    fn standard_curve_monotonic() {
        let mut prev = -1.0;
        for i in 0..=100 {
            let t = i as f64 / 100.0;
            let v = STANDARD.sample(t);
            assert!(v >= prev, "STANDARD regressed at t={t}");
            prev = v;
        }
    }

    #[test]
    fn emphasized_passes_through_midpoint() {
        // 中点 (1/6, 0.4)：两段曲线在该处拼接且取值连续
        assert!(close(EMPHASIZED.sample(0.166_666), 0.4));
        assert!(EMPHASIZED.sample(0.166_666 - 1e-4) < 0.4);
        assert!(EMPHASIZED.sample(0.166_666 + 1e-4) > 0.4);
    }

    #[test]
    fn emphasized_monotonic() {
        let mut prev = -1.0;
        for i in 0..=200 {
            let t = i as f64 / 200.0;
            let v = EMPHASIZED.sample(t);
            assert!(v >= prev - 1e-9, "EMPHASIZED regressed at t={t}");
            prev = v;
        }
    }
}
