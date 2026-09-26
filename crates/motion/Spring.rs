//! 阻尼弹簧求解器（单位质量）。
//!
//! 移植自 [m3fx](https://github.com/Glavo/m3fx) 的
//! `org.glavo.m3fx.internal.M3SpringSolver`（Apache-2.0，© 2026 Glavo）。
//! 使用解析闭式解直接求值（无数值积分），与 Compose 的 spring
//! 模型一致；仅时长估算使用牛顿迭代。

/// 弹簧物理参数：阻尼比与刚度（单位质量）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpringParameters {
    /// 阻尼比 ζ：<1 欠阻尼（会过冲）、=1 临界阻尼、>1 过阻尼。
    pub damping_ratio: f64,
    /// 刚度 k（N/m）；固有频率 ωₙ = √k。
    pub stiffness: f64,
}

impl SpringParameters {
    /// 创建弹簧参数。
    pub const fn new(damping_ratio: f64, stiffness: f64) -> Self {
        Self {
            damping_ratio,
            stiffness,
        }
    }

    /// 判定弹簧是否最终会收敛到目标（非零刚度且阻尼比有限）。
    pub fn is_sensible(&self) -> bool {
        self.stiffness > 0.0 && self.damping_ratio >= 0.0 && self.damping_ratio.is_finite()
    }
}

/// 判定阻尼区间用的容差（与 m3fx 一致）。
const CRITICAL_TOLERANCE: f64 = 1.0e-6;
/// 时长估算的牛顿迭代上限。
const MAX_DURATION_ITERATIONS: usize = 100;
/// 时长估算的收敛容差（秒）。
const DURATION_CONVERGENCE: f64 = 1.0e-3;

/// 计算弹簧在 `elapsed_seconds` 时刻的位移。
///
/// `start`/`target` 为起止值，`initial_velocity` 为初速度（值/秒）。
/// 返回 `target + displacement(start - target, ...)`，即当前值。
pub fn spring_value(
    start: f64,
    target: f64,
    initial_velocity: f64,
    elapsed_seconds: f64,
    spring: SpringParameters,
) -> f64 {
    target + spring_displacement(start - target, initial_velocity, elapsed_seconds, spring)
}

/// 计算弹簧在 `elapsed_seconds` 时刻的速度（值/秒）。
pub fn spring_velocity(
    start: f64,
    target: f64,
    initial_velocity: f64,
    elapsed_seconds: f64,
    spring: SpringParameters,
) -> f64 {
    spring_displacement_velocity(start - target, initial_velocity, elapsed_seconds, spring)
}

/// 估算弹簧从初位移收敛到 `visibility_threshold` 以内所需的时长（秒）。
///
/// 估算的是位移包络最后一次穿过阈值的时间（与 Compose 的估算模型一致）。
/// 无法估算时返回 `f64::NAN`，调用方应回退到 spec 的 fallback 时长。
pub fn estimate_duration_seconds(
    initial_displacement: f64,
    initial_velocity: f64,
    visibility_threshold: f64,
    spring: SpringParameters,
) -> f64 {
    if !(spring.is_sensible()
        && initial_displacement.is_finite()
        && initial_velocity.is_finite()
        && visibility_threshold.is_finite()
        && visibility_threshold > 0.0)
    {
        return f64::NAN;
    }
    let x0 = initial_displacement.abs();
    if x0 <= visibility_threshold {
        return 0.0;
    }

    let omega_n = spring.stiffness.sqrt();
    let zeta = spring.damping_ratio;
    let v0 = initial_velocity;

    if zeta < 1.0 - CRITICAL_TOLERANCE {
        // 欠阻尼：位移被指数衰减包络 R·e^(-ζωₙt) 界定，
        // 包络降到阈值之下的时刻即估算时长。
        let omega_d = omega_n * (1.0 - zeta * zeta).sqrt();
        let c2 = (v0 + zeta * omega_n * x0) / omega_d;
        let r = (x0 * x0 + c2 * c2).sqrt();
        if r <= 0.0 {
            return f64::NAN;
        }
        (r / visibility_threshold).ln() / (zeta * omega_n)
    } else if zeta > 1.0 + CRITICAL_TOLERANCE {
        // 过阻尼：对 |x(t)| - threshold 做牛顿迭代。
        let r1 = -omega_n * (zeta - (zeta * zeta - 1.0).sqrt());
        let r2 = -omega_n * (zeta + (zeta * zeta - 1.0).sqrt());
        if (r1 - r2).abs() < 1e-12 {
            return f64::NAN;
        }
        let c1 = (v0 - r2 * x0) / (r1 - r2);
        let c2 = x0 - c1;
        let value = |t: f64| c1 * exp_to(r1, t) + c2 * exp_to(r2, t);
        let dvalue = |t: f64| c1 * r1 * exp_to(r1, t) + c2 * r2 * exp_to(r2, t);
        newton_duration(x0, visibility_threshold, value, dvalue)
    } else {
        // 临界阻尼：x(t) = (x0 + (v0 + ωₙx0)·t)·e^(-ωₙt)
        let value = |t: f64| (x0 + (v0 + omega_n * x0) * t) * exp_to(-omega_n, t);
        let dvalue = |t: f64| exp_to(-omega_n, t) * (v0 - omega_n * (v0 + omega_n * x0) * t);
        newton_duration(x0, visibility_threshold, value, dvalue)
    }
}

/// `e^(rate·t)` 的薄包装，便于阅读。
fn exp_to(rate: f64, t: f64) -> f64 {
    (rate * t).exp()
}

/// 用牛顿迭代求 |value(t)| 首次降到阈值的时间。
fn newton_duration(
    x0: f64,
    threshold: f64,
    value: impl Fn(f64) -> f64,
    dvalue: impl Fn(f64) -> f64,
) -> f64 {
    // 以指数衰减近似（忽略速度项）做初值猜测
    let mut t = (x0 / threshold).ln().max(0.0);
    let mut i = 0;
    while i < MAX_DURATION_ITERATIONS {
        let f = value(t).abs() - threshold;
        if f <= 0.0 {
            return t;
        }
        let df = dvalue(t).abs();
        if df <= 1e-12 || !df.is_finite() {
            return f64::NAN;
        }
        let next = t - f / df;
        if !next.is_finite() || next < 0.0 {
            return f64::NAN;
        }
        if (next - t).abs() < DURATION_CONVERGENCE {
            return next;
        }
        t = next;
        i += 1;
    }
    f64::NAN
}

/// 计算位移 `x(t)`（以目标为原点）。
fn spring_displacement(x0: f64, v0: f64, t: f64, spring: SpringParameters) -> f64 {
    if t <= 0.0 {
        return x0;
    }
    let omega_n = spring.stiffness.sqrt();
    let zeta = spring.damping_ratio;
    if zeta < 1.0 - CRITICAL_TOLERANCE {
        // 欠阻尼：x(t) = e^(-ζωₙt)·(x0·cos(ω_d t) + c2·sin(ω_d t))
        let omega_d = omega_n * (1.0 - zeta * zeta).sqrt();
        let c2 = (v0 + zeta * omega_n * x0) / omega_d;
        let decay = exp_to(-zeta * omega_n, t);
        decay * (x0 * (omega_d * t).cos() + c2 * (omega_d * t).sin())
    } else if zeta > 1.0 + CRITICAL_TOLERANCE {
        // 过阻尼：x(t) = c1·e^(r1 t) + c2·e^(r2 t)
        let r1 = -omega_n * (zeta - (zeta * zeta - 1.0).sqrt());
        let r2 = -omega_n * (zeta + (zeta * zeta - 1.0).sqrt());
        if (r1 - r2).abs() < 1e-12 {
            return 0.0;
        }
        let c1 = (v0 - r2 * x0) / (r1 - r2);
        let c2 = x0 - c1;
        c1 * exp_to(r1, t) + c2 * exp_to(r2, t)
    } else {
        // 临界阻尼：x(t) = (x0 + (v0 + ωₙx0)·t)·e^(-ωₙt)
        (x0 + (v0 + omega_n * x0) * t) * exp_to(-omega_n, t)
    }
}

/// 计算位移对时间的导数 `x'(t)`（以目标为原点）。
fn spring_displacement_velocity(x0: f64, v0: f64, t: f64, spring: SpringParameters) -> f64 {
    if t <= 0.0 {
        return v0;
    }
    let omega_n = spring.stiffness.sqrt();
    let zeta = spring.damping_ratio;
    if zeta < 1.0 - CRITICAL_TOLERANCE {
        let omega_d = omega_n * (1.0 - zeta * zeta).sqrt();
        let c2 = (v0 + zeta * omega_n * x0) / omega_d;
        let decay = exp_to(-zeta * omega_n, t);
        let (st, ct) = (omega_d * t).sin_cos();
        decay * (-x0 * omega_d * st + c2 * omega_d * ct - zeta * omega_n * (x0 * ct + c2 * st))
    } else if zeta > 1.0 + CRITICAL_TOLERANCE {
        let r1 = -omega_n * (zeta - (zeta * zeta - 1.0).sqrt());
        let r2 = -omega_n * (zeta + (zeta * zeta - 1.0).sqrt());
        if (r1 - r2).abs() < 1e-12 {
            return 0.0;
        }
        let c1 = (v0 - r2 * x0) / (r1 - r2);
        let c2 = x0 - c1;
        c1 * r1 * exp_to(r1, t) + c2 * r2 * exp_to(r2, t)
    } else {
        exp_to(-omega_n, t) * (v0 - omega_n * (v0 + omega_n * x0) * t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPRING: SpringParameters = SpringParameters::new(0.9, 700.0);

    fn close(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() < eps
    }

    #[test]
    fn starts_at_start() {
        assert!(close(spring_value(10.0, 0.0, 0.0, 0.0, SPRING), 10.0, 1e-9));
    }

    #[test]
    fn converges_to_target() {
        let v = spring_value(0.0, 1.0, 0.0, 5.0, SPRING);
        assert!(close(v, 1.0, 1e-9), "v={v}");
    }

    #[test]
    fn energy_dissipates() {
        // 欠阻尼弹簧的峰值速度应随时间衰减
        let mut max_v = 0.0;
        for i in 0..20 {
            let t = i as f64 * 0.05;
            let v = spring_velocity(0.0, 1.0, 0.0, t, SPRING).abs();
            if i > 2 {
                assert!(v <= max_v + 1e-6, "velocity grew at t={t}");
            }
            max_v = max_v.max(v);
        }
    }

    #[test]
    fn velocity_discontinuity_free_retarget() {
        // 从当前速度续算，值应连续
        let mid_value = spring_value(0.0, 1.0, 0.0, 0.1, SPRING);
        let mid_velocity = spring_velocity(0.0, 1.0, 0.0, 0.1, SPRING);
        // 把 mid_value 当作新起点、保留速度继续运动，短时间后与原始轨迹一致
        let a = spring_value(0.0, 1.0, 0.0, 0.15, SPRING);
        let b = spring_value(mid_value, 1.0, mid_velocity, 0.05, SPRING);
        assert!(close(a, b, 1e-9), "a={a} b={b}");
    }

    #[test]
    fn critical_damping_converges() {
        let spring = SpringParameters::new(1.0, 400.0);
        let v = spring_value(0.0, 1.0, 0.0, 2.0, spring);
        assert!(close(v, 1.0, 1e-9), "v={v}");
    }

    #[test]
    fn overdamping_converges() {
        let spring = SpringParameters::new(1.8, 400.0);
        let v = spring_value(0.0, 1.0, 0.0, 5.0, spring);
        assert!(close(v, 1.0, 1e-9), "v={v}");
    }

    #[test]
    fn duration_estimate_reasonable() {
        let d = estimate_duration_seconds(1.0, 0.0, 0.001, SPRING);
        assert!(d.is_finite() && d > 0.0 && d < 10.0, "d={d}");
        let v = spring_value(0.0, 1.0, 0.0, d, SPRING);
        assert!((v - 1.0).abs() <= 0.001 * 1.5, "v={v} at d={d}");
    }

    #[test]
    fn duration_estimate_nan_for_bad_input() {
        assert!(estimate_duration_seconds(f64::NAN, 0.0, 0.001, SPRING).is_nan());
        assert!(
            estimate_duration_seconds(1.0, 0.0, -1.0, SPRING).is_nan(),
            "负阈值应返回 NaN"
        );
    }
}
