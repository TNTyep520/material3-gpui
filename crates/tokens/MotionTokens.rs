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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Tokens.kt

use super::{TokenEntry, TokenValue};
use crate::motion::Easing;
use std::time::Duration;

#[derive(Clone, Copy, Debug, Default)]
pub struct MotionTokens;
impl MotionTokens {
    pub const DURATION_EXTRA_LONG1: f64 = 700.0;
    pub const DURATION_EXTRA_LONG2: f64 = 800.0;
    pub const DURATION_EXTRA_LONG3: f64 = 900.0;
    pub const DURATION_EXTRA_LONG4: f64 = 1000.0;
    pub const DURATION_LONG1: f64 = 450.0;
    pub const DURATION_LONG2: f64 = 500.0;
    pub const DURATION_LONG3: f64 = 550.0;
    pub const DURATION_LONG4: f64 = 600.0;
    pub const DURATION_MEDIUM1: f64 = 250.0;
    pub const DURATION_MEDIUM2: f64 = 300.0;
    pub const DURATION_MEDIUM3: f64 = 350.0;
    pub const DURATION_MEDIUM4: f64 = 400.0;
    pub const DURATION_SHORT1: f64 = 50.0;
    pub const DURATION_SHORT2: f64 = 100.0;
    pub const DURATION_SHORT3: f64 = 150.0;
    pub const DURATION_SHORT4: f64 = 200.0;
    pub const EASING_EMPHASIZED_CUBIC_BEZIER: Easing = Easing::CubicBezier {
        x1: 0.2,
        y1: 0.0,
        x2: 0.0,
        y2: 1.0,
    };
    pub const EASING_EMPHASIZED_ACCELERATE_CUBIC_BEZIER: Easing = Easing::CubicBezier {
        x1: 0.3,
        y1: 0.0,
        x2: 0.8,
        y2: 0.15,
    };
    pub const EASING_EMPHASIZED_DECELERATE_CUBIC_BEZIER: Easing = Easing::CubicBezier {
        x1: 0.05,
        y1: 0.7,
        x2: 0.1,
        y2: 1.0,
    };
    pub const EASING_LEGACY_CUBIC_BEZIER: Easing = Easing::CubicBezier {
        x1: 0.4,
        y1: 0.0,
        x2: 0.2,
        y2: 1.0,
    };
    pub const EASING_LEGACY_ACCELERATE_CUBIC_BEZIER: Easing = Easing::CubicBezier {
        x1: 0.4,
        y1: 0.0,
        x2: 1.0,
        y2: 1.0,
    };
    pub const EASING_LEGACY_DECELERATE_CUBIC_BEZIER: Easing = Easing::CubicBezier {
        x1: 0.0,
        y1: 0.0,
        x2: 0.2,
        y2: 1.0,
    };
    pub const EASING_LINEAR_CUBIC_BEZIER: Easing = Easing::CubicBezier {
        x1: 0.0,
        y1: 0.0,
        x2: 1.0,
        y2: 1.0,
    };
    pub const EASING_STANDARD_CUBIC_BEZIER: Easing = Easing::CubicBezier {
        x1: 0.2,
        y1: 0.0,
        x2: 0.0,
        y2: 1.0,
    };
    pub const EASING_STANDARD_ACCELERATE_CUBIC_BEZIER: Easing = Easing::CubicBezier {
        x1: 0.3,
        y1: 0.0,
        x2: 1.0,
        y2: 1.0,
    };
    pub const EASING_STANDARD_DECELERATE_CUBIC_BEZIER: Easing = Easing::CubicBezier {
        x1: 0.0,
        y1: 0.0,
        x2: 0.0,
        y2: 1.0,
    };
}
impl MotionTokens {
    pub const EXTRA_LONG1: Duration = Duration::from_millis(700);
    pub const EXTRA_LONG2: Duration = Duration::from_millis(800);
    pub const EXTRA_LONG3: Duration = Duration::from_millis(900);
    pub const EXTRA_LONG4: Duration = Duration::from_millis(1000);
    pub const LONG1: Duration = Duration::from_millis(450);
    pub const LONG2: Duration = Duration::from_millis(500);
    pub const LONG3: Duration = Duration::from_millis(550);
    pub const LONG4: Duration = Duration::from_millis(600);
    pub const MEDIUM1: Duration = Duration::from_millis(250);
    pub const MEDIUM2: Duration = Duration::from_millis(300);
    pub const MEDIUM3: Duration = Duration::from_millis(350);
    pub const MEDIUM4: Duration = Duration::from_millis(400);
    pub const SHORT1: Duration = Duration::from_millis(50);
    pub const SHORT2: Duration = Duration::from_millis(100);
    pub const SHORT3: Duration = Duration::from_millis(150);
    pub const SHORT4: Duration = Duration::from_millis(200);
}
impl MotionTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "MotionTokens",
            name: "DurationExtraLong1",
            value: TokenValue::Double(MotionTokens::DURATION_EXTRA_LONG1),
        },
        TokenEntry {
            group: "MotionTokens",
            name: "DurationExtraLong2",
            value: TokenValue::Double(MotionTokens::DURATION_EXTRA_LONG2),
        },
        TokenEntry {
            group: "MotionTokens",
            name: "DurationExtraLong3",
            value: TokenValue::Double(MotionTokens::DURATION_EXTRA_LONG3),
        },
        TokenEntry {
            group: "MotionTokens",
            name: "DurationExtraLong4",
            value: TokenValue::Double(MotionTokens::DURATION_EXTRA_LONG4),
        },
        TokenEntry {
            group: "MotionTokens",
            name: "DurationLong1",
            value: TokenValue::Double(MotionTokens::DURATION_LONG1),
        },
        TokenEntry {
            group: "MotionTokens",
            name: "DurationLong2",
            value: TokenValue::Double(MotionTokens::DURATION_LONG2),
        },
        TokenEntry {
            group: "MotionTokens",
            name: "DurationLong3",
            value: TokenValue::Double(MotionTokens::DURATION_LONG3),
        },
        TokenEntry {
            group: "MotionTokens",
            name: "DurationLong4",
            value: TokenValue::Double(MotionTokens::DURATION_LONG4),
        },
        TokenEntry {
            group: "MotionTokens",
            name: "DurationMedium1",
            value: TokenValue::Double(MotionTokens::DURATION_MEDIUM1),
        },
        TokenEntry {
            group: "MotionTokens",
            name: "DurationMedium2",
            value: TokenValue::Double(MotionTokens::DURATION_MEDIUM2),
        },
        TokenEntry {
            group: "MotionTokens",
            name: "DurationMedium3",
            value: TokenValue::Double(MotionTokens::DURATION_MEDIUM3),
        },
        TokenEntry {
            group: "MotionTokens",
            name: "DurationMedium4",
            value: TokenValue::Double(MotionTokens::DURATION_MEDIUM4),
        },
        TokenEntry {
            group: "MotionTokens",
            name: "DurationShort1",
            value: TokenValue::Double(MotionTokens::DURATION_SHORT1),
        },
        TokenEntry {
            group: "MotionTokens",
            name: "DurationShort2",
            value: TokenValue::Double(MotionTokens::DURATION_SHORT2),
        },
        TokenEntry {
            group: "MotionTokens",
            name: "DurationShort3",
            value: TokenValue::Double(MotionTokens::DURATION_SHORT3),
        },
        TokenEntry {
            group: "MotionTokens",
            name: "DurationShort4",
            value: TokenValue::Double(MotionTokens::DURATION_SHORT4),
        },
        TokenEntry {
            group: "MotionTokens",
            name: "EasingEmphasizedCubicBezier",
            value: TokenValue::Easing(MotionTokens::EASING_EMPHASIZED_CUBIC_BEZIER),
        },
        TokenEntry {
            group: "MotionTokens",
            name: "EasingEmphasizedAccelerateCubicBezier",
            value: TokenValue::Easing(MotionTokens::EASING_EMPHASIZED_ACCELERATE_CUBIC_BEZIER),
        },
        TokenEntry {
            group: "MotionTokens",
            name: "EasingEmphasizedDecelerateCubicBezier",
            value: TokenValue::Easing(MotionTokens::EASING_EMPHASIZED_DECELERATE_CUBIC_BEZIER),
        },
        TokenEntry {
            group: "MotionTokens",
            name: "EasingLegacyCubicBezier",
            value: TokenValue::Easing(MotionTokens::EASING_LEGACY_CUBIC_BEZIER),
        },
        TokenEntry {
            group: "MotionTokens",
            name: "EasingLegacyAccelerateCubicBezier",
            value: TokenValue::Easing(MotionTokens::EASING_LEGACY_ACCELERATE_CUBIC_BEZIER),
        },
        TokenEntry {
            group: "MotionTokens",
            name: "EasingLegacyDecelerateCubicBezier",
            value: TokenValue::Easing(MotionTokens::EASING_LEGACY_DECELERATE_CUBIC_BEZIER),
        },
        TokenEntry {
            group: "MotionTokens",
            name: "EasingLinearCubicBezier",
            value: TokenValue::Easing(MotionTokens::EASING_LINEAR_CUBIC_BEZIER),
        },
        TokenEntry {
            group: "MotionTokens",
            name: "EasingStandardCubicBezier",
            value: TokenValue::Easing(MotionTokens::EASING_STANDARD_CUBIC_BEZIER),
        },
        TokenEntry {
            group: "MotionTokens",
            name: "EasingStandardAccelerateCubicBezier",
            value: TokenValue::Easing(MotionTokens::EASING_STANDARD_ACCELERATE_CUBIC_BEZIER),
        },
        TokenEntry {
            group: "MotionTokens",
            name: "EasingStandardDecelerateCubicBezier",
            value: TokenValue::Easing(MotionTokens::EASING_STANDARD_DECELERATE_CUBIC_BEZIER),
        },
    ];
}
