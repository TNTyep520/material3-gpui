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

#[derive(Clone, Copy, Debug, Default)]
pub struct StandardMotionTokens;
impl StandardMotionTokens {
    pub const SPRING_DEFAULT_SPATIAL_DAMPING: f32 = 0.9;
    pub const SPRING_DEFAULT_SPATIAL_STIFFNESS: f32 = 700.0;
    pub const SPRING_DEFAULT_EFFECTS_DAMPING: f32 = 1.0;
    pub const SPRING_DEFAULT_EFFECTS_STIFFNESS: f32 = 1600.0;
    pub const SPRING_FAST_SPATIAL_DAMPING: f32 = 0.9;
    pub const SPRING_FAST_SPATIAL_STIFFNESS: f32 = 1400.0;
    pub const SPRING_FAST_EFFECTS_DAMPING: f32 = 1.0;
    pub const SPRING_FAST_EFFECTS_STIFFNESS: f32 = 3800.0;
    pub const SPRING_SLOW_SPATIAL_DAMPING: f32 = 0.9;
    pub const SPRING_SLOW_SPATIAL_STIFFNESS: f32 = 300.0;
    pub const SPRING_SLOW_EFFECTS_DAMPING: f32 = 1.0;
    pub const SPRING_SLOW_EFFECTS_STIFFNESS: f32 = 800.0;
}
impl StandardMotionTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "StandardMotionTokens",
            name: "SpringDefaultSpatialDamping",
            value: TokenValue::Float(StandardMotionTokens::SPRING_DEFAULT_SPATIAL_DAMPING),
        },
        TokenEntry {
            group: "StandardMotionTokens",
            name: "SpringDefaultSpatialStiffness",
            value: TokenValue::Float(StandardMotionTokens::SPRING_DEFAULT_SPATIAL_STIFFNESS),
        },
        TokenEntry {
            group: "StandardMotionTokens",
            name: "SpringDefaultEffectsDamping",
            value: TokenValue::Float(StandardMotionTokens::SPRING_DEFAULT_EFFECTS_DAMPING),
        },
        TokenEntry {
            group: "StandardMotionTokens",
            name: "SpringDefaultEffectsStiffness",
            value: TokenValue::Float(StandardMotionTokens::SPRING_DEFAULT_EFFECTS_STIFFNESS),
        },
        TokenEntry {
            group: "StandardMotionTokens",
            name: "SpringFastSpatialDamping",
            value: TokenValue::Float(StandardMotionTokens::SPRING_FAST_SPATIAL_DAMPING),
        },
        TokenEntry {
            group: "StandardMotionTokens",
            name: "SpringFastSpatialStiffness",
            value: TokenValue::Float(StandardMotionTokens::SPRING_FAST_SPATIAL_STIFFNESS),
        },
        TokenEntry {
            group: "StandardMotionTokens",
            name: "SpringFastEffectsDamping",
            value: TokenValue::Float(StandardMotionTokens::SPRING_FAST_EFFECTS_DAMPING),
        },
        TokenEntry {
            group: "StandardMotionTokens",
            name: "SpringFastEffectsStiffness",
            value: TokenValue::Float(StandardMotionTokens::SPRING_FAST_EFFECTS_STIFFNESS),
        },
        TokenEntry {
            group: "StandardMotionTokens",
            name: "SpringSlowSpatialDamping",
            value: TokenValue::Float(StandardMotionTokens::SPRING_SLOW_SPATIAL_DAMPING),
        },
        TokenEntry {
            group: "StandardMotionTokens",
            name: "SpringSlowSpatialStiffness",
            value: TokenValue::Float(StandardMotionTokens::SPRING_SLOW_SPATIAL_STIFFNESS),
        },
        TokenEntry {
            group: "StandardMotionTokens",
            name: "SpringSlowEffectsDamping",
            value: TokenValue::Float(StandardMotionTokens::SPRING_SLOW_EFFECTS_DAMPING),
        },
        TokenEntry {
            group: "StandardMotionTokens",
            name: "SpringSlowEffectsStiffness",
            value: TokenValue::Float(StandardMotionTokens::SPRING_SLOW_EFFECTS_STIFFNESS),
        },
    ];
}
