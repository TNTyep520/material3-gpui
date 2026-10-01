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
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/animation/M3MotionEasing.java

use std::time::Duration;

use crate::tokens::MotionTokens;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Easing {
    Linear,

    CubicBezier { x1: f64, y1: f64, x2: f64, y2: f64 },

    Emphasized,
}

impl Easing {
    pub fn sample(&self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        match *self {
            Easing::Linear => t,
            Easing::CubicBezier { x1, y1, x2, y2 } => unit_cubic_y(t, x1, y1, x2, y2),
            Easing::Emphasized => {
                const MID_X: f64 = 0.166_666;
                const MID_Y: f64 = 0.4;
                if t < MID_X {
                    segment_y(t, MID_X, MID_Y, 0.0, 0.0, 0.05, 0.0, 0.133_333, 0.06)
                } else {
                    segment_y(t, 1.0, 1.0, MID_X, MID_Y, 0.208_333, 0.82, 0.25, 1.0)
                }
            }
        }
    }

    pub fn sample_duration(&self, elapsed: Duration, duration: Duration) -> f64 {
        if duration.is_zero() {
            return 1.0;
        }
        let t = elapsed.as_secs_f64() / duration.as_secs_f64();
        self.sample(t)
    }
}

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

pub const LINEAR: Easing = Easing::Linear;

pub const STANDARD: Easing = MotionTokens::EASING_STANDARD_CUBIC_BEZIER;

pub const STANDARD_ACCELERATE: Easing = MotionTokens::EASING_STANDARD_ACCELERATE_CUBIC_BEZIER;

pub const STANDARD_DECELERATE: Easing = MotionTokens::EASING_STANDARD_DECELERATE_CUBIC_BEZIER;

pub const EMPHASIZED_ACCELERATE: Easing = MotionTokens::EASING_EMPHASIZED_ACCELERATE_CUBIC_BEZIER;

pub const EMPHASIZED_DECELERATE: Easing = MotionTokens::EASING_EMPHASIZED_DECELERATE_CUBIC_BEZIER;

pub const STANDARD_SPATIAL: Easing = Easing::CubicBezier {
    x1: 0.27,
    y1: 1.06,
    x2: 0.18,
    y2: 1.0,
};

pub const EXPRESSIVE_FAST_SPATIAL: Easing = Easing::CubicBezier {
    x1: 0.42,
    y1: 1.67,
    x2: 0.21,
    y2: 0.9,
};

pub const EXPRESSIVE_DEFAULT_SPATIAL: Easing = Easing::CubicBezier {
    x1: 0.38,
    y1: 1.21,
    x2: 0.22,
    y2: 1.0,
};

pub const EXPRESSIVE_SLOW_SPATIAL: Easing = Easing::CubicBezier {
    x1: 0.39,
    y1: 1.29,
    x2: 0.35,
    y2: 0.98,
};

pub const FAST_EFFECTS: Easing = Easing::CubicBezier {
    x1: 0.31,
    y1: 0.94,
    x2: 0.34,
    y2: 1.0,
};

pub const DEFAULT_EFFECTS: Easing = Easing::CubicBezier {
    x1: 0.34,
    y1: 0.8,
    x2: 0.34,
    y2: 1.0,
};

pub const SLOW_EFFECTS: Easing = Easing::CubicBezier {
    x1: 0.34,
    y1: 0.88,
    x2: 0.34,
    y2: 1.0,
};

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
