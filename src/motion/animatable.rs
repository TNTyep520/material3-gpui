//! 可动画值运行时：弹簧驱动的可重定向数值与组件逐帧驱动器。
//!
//! 对应 [m3fx](https://github.com/Glavo/m3fx) 的 `M3DoubleAnimatable`
//! 与 `M3StateTransition`（Apache-2.0，© 2026 Glavo）。
//!
//! 语义与 m3fx 一致：
//! - 中途改目标（retarget）保留当前速度，不产生速度突变；
//! - 弹簧时长由可视阈值（visibility threshold）估算，只决定何时判定收敛，
//!   不会对最终值做截断；
//! - 弹簧估算失败时回退到 spec 的 fallback 时长/缓动。

use std::time::{Duration, Instant};

use gpui::{Context, Entity, Hsla, Render, Rgba, Window};

use super::easing::Easing;
use super::scheme::MotionSpec;
use super::spring::{SpringParameters, estimate_duration_seconds, spring_value, spring_velocity};

/// 单个弹簧/时长动画的运行状态。
#[derive(Clone, Copy, Debug)]
enum Run {
    /// 弹簧运行：记录起点、初速度、开始时刻与预计收敛时长。
    Spring {
        start_value: f64,
        start_velocity: f64,
        started_at: Instant,
        /// 估算收敛时长（秒）。
        duration_s: f64,
        params: SpringParameters,
    },
    /// 时长动画：固定时长 + 缓动。
    Timed {
        start_value: f64,
        started_at: Instant,
        duration: Duration,
        easing: Easing,
    },
}

impl Run {
    fn started_at(&self) -> Instant {
        match *self {
            Run::Spring { started_at, .. } | Run::Timed { started_at, .. } => started_at,
        }
    }

    fn duration(&self) -> Duration {
        match *self {
            Run::Spring { duration_s, .. } => Duration::from_secs_f64(duration_s.max(0.0)),
            Run::Timed { duration, .. } => duration,
        }
    }
}

/// 可重定向的可动画标量值。
///
/// 组件把它作为字段持有，在交互时调用 [`Animatable::animate_to`]，
/// 在 render 中通过 [`AnimationDriver`] 逐帧推进。
#[derive(Clone, Copy, Debug)]
pub struct Animatable {
    value: f64,
    velocity: f64,
    target: f64,
    visibility_threshold: f64,
    run: Option<Run>,
}

impl Animatable {
    /// 创建静止在 `value` 上的可动画值。
    ///
    /// `visibility_threshold` 是弹簧判定"视觉上已收敛"的有限正值增量，
    /// 用于估算弹簧时长（见 m3fx `M3DoubleAnimatable`）。
    pub fn new(value: f64, visibility_threshold: f64) -> Self {
        Self {
            value,
            velocity: 0.0,
            target: value,
            visibility_threshold: visibility_threshold.max(f64::MIN_POSITIVE),
            run: None,
        }
    }

    /// 当前值。
    pub fn value(&self) -> f64 {
        self.value
    }

    /// 最近一次请求的目标值。
    pub fn target(&self) -> f64 {
        self.target
    }

    /// 当前速度（值/秒），供连续 retarget 或测试使用。
    pub fn velocity(&self) -> f64 {
        self.velocity
    }

    /// 是否仍在动画中。
    pub fn is_running(&self) -> bool {
        self.run.is_some()
    }

    /// 以弹簧动画驶向 `target`（使用 `spec` 的弹簧参数与回退值）。
    ///
    /// 若正在动画中，则从当前值/速度继续（保留速度，无突变）。
    pub fn animate_to(&mut self, target: f64, spec: &MotionSpec, now: Instant) {
        self.animate_to_with_params(
            target,
            spec.spring,
            spec.fallback_duration,
            spec.fallback_easing,
            now,
        );
    }

    /// 以自定义弹簧参数驶向 `target`；估算失败时用给定的回退时长/缓动。
    pub fn animate_to_with_params(
        &mut self,
        target: f64,
        params: SpringParameters,
        fallback_duration: Duration,
        fallback_easing: Easing,
        now: Instant,
    ) {
        self.sync(now);
        self.target = target;
        if (target - self.value).abs() <= self.visibility_threshold {
            self.value = target;
            self.velocity = 0.0;
            self.run = None;
            return;
        }
        if !params.is_sensible() {
            self.start_timed(target, fallback_duration, fallback_easing, now);
            return;
        }
        let duration_s = estimate_duration_seconds(
            target - self.value,
            self.velocity,
            self.visibility_threshold,
            params,
        );
        if duration_s.is_finite() {
            self.run = Some(Run::Spring {
                start_value: self.value,
                start_velocity: self.velocity,
                started_at: now,
                duration_s,
                params,
            });
        } else {
            self.start_timed(target, fallback_duration, fallback_easing, now);
        }
    }

    /// 以固定时长 + 缓动驶向 `target`。
    pub fn animate_to_timed(
        &mut self,
        target: f64,
        duration: Duration,
        easing: Easing,
        now: Instant,
    ) {
        self.sync(now);
        self.target = target;
        self.start_timed(target, duration, easing, now);
    }

    /// 停止动画并直接落到 `value`（当前值与目标值同时被设置）。
    pub fn snap_to(&mut self, value: f64) {
        self.run = None;
        self.value = value;
        self.target = value;
        self.velocity = 0.0;
    }

    /// 冻结在当前值（目标值保持不变）。
    pub fn stop(&mut self) {
        self.run = None;
    }

    /// 立刻完成：跳到目标值并停止（幂等）。
    pub fn finish(&mut self) {
        self.value = self.target;
        self.velocity = 0.0;
        self.run = None;
    }

    /// 推进到 `now`；返回推进后是否仍在动画中。
    pub fn tick(&mut self, now: Instant) -> bool {
        self.sync(now);
        self.run.is_some()
    }

    /// 把内部状态推进到 `now` 时刻（若在动画中）。
    fn sync(&mut self, now: Instant) {
        let Some(run) = self.run else {
            return;
        };
        let elapsed = now.saturating_duration_since(run.started_at());
        let duration = run.duration();
        if elapsed >= duration {
            self.value = self.target;
            self.velocity = 0.0;
            self.run = None;
            return;
        }
        match run {
            Run::Spring {
                start_value,
                start_velocity,
                params,
                ..
            } => {
                self.value = spring_value(
                    start_value,
                    self.target,
                    start_velocity,
                    elapsed.as_secs_f64(),
                    params,
                );
                self.velocity = spring_velocity(
                    start_value,
                    self.target,
                    start_velocity,
                    elapsed.as_secs_f64(),
                    params,
                );
            }
            Run::Timed {
                start_value,
                easing,
                ..
            } => {
                let progress = easing.sample_duration(elapsed, duration);
                self.value = start_value + (self.target - start_value) * progress;
                // 数值微分估计当前速度，供后续弹簧 retarget 使用
                let dt = 1.0e-3_f64;
                let p1 = easing.sample_duration(elapsed + Duration::from_secs_f64(dt), duration);
                self.velocity = (p1 - progress) * (self.target - start_value) / dt;
            }
        }
    }

    /// 启动时长动画（内部：假定 sync 已调用、target 已更新）。
    fn start_timed(&mut self, target: f64, duration: Duration, easing: Easing, now: Instant) {
        self.target = target;
        if duration.is_zero() {
            self.value = target;
            self.velocity = 0.0;
            self.run = None;
            return;
        }
        self.run = Some(Run::Timed {
            start_value: self.value,
            started_at: now,
            duration,
            easing,
        });
    }
}

/// 组件内嵌的逐帧驱动器：防止同一帧重复调度。
#[derive(Clone, Copy, Debug, Default)]
pub struct AnimationDriver {
    scheduled: bool,
}

impl AnimationDriver {
    /// 在组件的 `render` 中调用：若组件仍有动画在跑，
    /// 安排在下一帧调用 [`AnimatedComponent::step`]。
    pub fn schedule<T: AnimatedComponent + 'static>(
        &mut self,
        entity: &Entity<T>,
        window: &mut Window,
    ) {
        if self.scheduled {
            return;
        }
        self.scheduled = true;
        let entity = entity.clone();
        window.on_next_frame(move |window, cx| {
            entity.update(cx, |component, cx| {
                let still_running = component.step(Instant::now());
                component.driver_mut().scheduled = false;
                if still_running {
                    component.schedule_next(window, cx);
                }
                // 动画推进与落地帧都需要一次 notify 重绘终值
                cx.notify();
            });
        });
    }
}

/// 支持逐帧动画的组件 trait。
///
/// 组件实现 [`AnimatedComponent::step`] 推进自身全部动画值；
/// 在 `render` 里调用 [`AnimatedComponent::schedule_next`] 即可维持动画循环。
pub trait AnimatedComponent: Render + Sized {
    /// 推进所有动画值到 `now`；返回是否仍有动画在运行。
    fn step(&mut self, now: Instant) -> bool;

    /// 返回内嵌的驱动器（用于维护调度标记）。
    fn driver_mut(&mut self) -> &mut AnimationDriver;

    /// 安排下一帧步进（render 中每帧调用一次即可）。
    fn schedule_next(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let entity = cx.entity();
        self.driver_mut().schedule(&entity, window);
    }
}

/// 在 sRGB 空间线性插值两个颜色（与 JavaFX `Color.interpolate` 行为一致）。
pub fn lerp_color(from: Hsla, to: Hsla, t: f32) -> Hsla {
    let from_rgba = Rgba::from(from);
    let to_rgba = Rgba::from(to);
    let mix = |a: f32, b: f32| a + (b - a) * t;
    Rgba {
        r: mix(from_rgba.r, to_rgba.r),
        g: mix(from_rgba.g, to_rgba.g),
        b: mix(from_rgba.b, to_rgba.b),
        a: mix(from_rgba.a, to_rgba.a),
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::motion::easing;
    use crate::motion::scheme::{MotionRole, MotionScheme};

    fn test_spec() -> MotionSpec {
        *MotionScheme::standard().spec(MotionRole::DefaultEffects)
    }

    #[test]
    fn snaps_for_tiny_delta() {
        let now = Instant::now();
        let mut a = Animatable::new(1.0, 0.01);
        a.animate_to(1.005, &test_spec(), now);
        assert!(!a.is_running());
        assert_eq!(a.value(), 1.005);
    }

    #[test]
    fn finishes_at_target() {
        let now = Instant::now();
        let mut a = Animatable::new(0.0, 1e-4);
        a.animate_to(1.0, &test_spec(), now);
        assert!(a.is_running());
        let _ = a.tick(now + Duration::from_secs(10));
        assert!(!a.is_running());
        assert_eq!(a.value(), 1.0);
        assert_eq!(a.target(), 1.0);
    }

    #[test]
    fn retarget_keeps_velocity() {
        let now = Instant::now();
        let mut a = Animatable::new(0.0, 1e-4);
        a.animate_to(1.0, &test_spec(), now);
        let mid = now + Duration::from_millis(50);
        a.tick(mid);
        assert!(
            a.velocity().abs() > 0.0,
            "mid-flight velocity should be non-zero"
        );
        let value_before = a.value();
        a.animate_to(2.0, &test_spec(), mid);
        // retarget 后立即取值应与 retarget 前一致（无跳变）
        assert_eq!(a.value(), value_before);
        assert_eq!(a.target(), 2.0);
    }

    #[test]
    fn timed_animation_runs_exactly() {
        let now = Instant::now();
        let mut a = Animatable::new(0.0, 1e-6);
        a.animate_to_timed(1.0, Duration::from_millis(100), easing::LINEAR, now);
        a.tick(now + Duration::from_millis(50));
        assert!((a.value() - 0.5).abs() < 1e-6, "v={}", a.value());
        a.tick(now + Duration::from_millis(150));
        assert_eq!(a.value(), 1.0);
    }

    #[test]
    fn color_lerp_endpoints() {
        let from: Hsla = gpui::rgb(0x000000).into();
        let to: Hsla = gpui::rgb(0xffffff).into();
        let mid_rgba: Rgba = lerp_color(from, to, 0.5).into();
        assert!((mid_rgba.r - 0.5).abs() < 1e-3);
    }
}
