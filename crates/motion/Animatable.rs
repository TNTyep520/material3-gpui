// Copyright (c) 2026 TNTyep520
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/animation/M3DoubleAnimatable.java

use std::time::{Duration, Instant};

use gpui::{Context, Entity, Hsla, Render, Rgba, Window};

use super::easing::Easing;
use super::scheme::MotionSpec;
use super::spring::{SpringParameters, estimate_duration_seconds, spring_value, spring_velocity};

#[derive(Clone, Copy, Debug)]
enum Run {
    Spring {
        start_value: f64,
        start_velocity: f64,
        started_at: Instant,

        duration_s: f64,
        params: SpringParameters,
    },

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

#[derive(Clone, Copy, Debug)]
pub struct Animatable {
    value: f64,
    velocity: f64,
    target: f64,
    visibility_threshold: f64,
    run: Option<Run>,
}

impl Animatable {
    pub fn new(value: f64, visibility_threshold: f64) -> Self {
        Self {
            value,
            velocity: 0.0,
            target: value,
            visibility_threshold: visibility_threshold.max(f64::MIN_POSITIVE),
            run: None,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn target(&self) -> f64 {
        self.target
    }

    pub fn velocity(&self) -> f64 {
        self.velocity
    }

    pub fn is_running(&self) -> bool {
        self.run.is_some()
    }

    pub fn animate_to(&mut self, target: f64, spec: &MotionSpec, now: Instant) {
        self.animate_to_with_params(
            target,
            spec.spring,
            spec.fallback_duration,
            spec.fallback_easing,
            now,
        );
    }

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

    pub fn snap_to(&mut self, value: f64) {
        self.run = None;
        self.value = value;
        self.target = value;
        self.velocity = 0.0;
    }

    pub fn stop(&mut self) {
        self.run = None;
    }

    pub fn finish(&mut self) {
        self.value = self.target;
        self.velocity = 0.0;
        self.run = None;
    }

    pub fn tick(&mut self, now: Instant) -> bool {
        self.sync(now);
        self.run.is_some()
    }

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

                let dt = 1.0e-3_f64;
                let p1 = easing.sample_duration(elapsed + Duration::from_secs_f64(dt), duration);
                self.velocity = (p1 - progress) * (self.target - start_value) / dt;
            }
        }
    }

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

#[derive(Clone, Copy, Debug, Default)]
pub struct AnimationDriver {
    scheduled: bool,
}

impl AnimationDriver {
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

                cx.notify();
            });
        });
    }
}

pub trait AnimatedComponent: Render + Sized {
    fn step(&mut self, now: Instant) -> bool;

    fn driver_mut(&mut self) -> &mut AnimationDriver;

    fn schedule_next(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let entity = cx.entity();
        self.driver_mut().schedule(&entity, window);
    }
}

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
