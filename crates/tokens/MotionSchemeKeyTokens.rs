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
use crate::motion::{MotionRole, MotionSpec};
use crate::theme::TokenSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MotionSchemeToken {
    DefaultSpatial,
    FastSpatial,
    SlowSpatial,
    DefaultEffects,
    FastEffects,
    SlowEffects,
}
impl MotionSchemeToken {
    pub const ALL: &'static [Self] = &[
        Self::DefaultSpatial,
        Self::FastSpatial,
        Self::SlowSpatial,
        Self::DefaultEffects,
        Self::FastEffects,
        Self::SlowEffects,
    ];
    pub const fn id(self) -> u16 {
        match self {
            Self::DefaultSpatial => 0,
            Self::FastSpatial => 1,
            Self::SlowSpatial => 2,
            Self::DefaultEffects => 3,
            Self::FastEffects => 4,
            Self::SlowEffects => 5,
        }
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct MotionSchemeKeyTokens;
impl MotionSchemeKeyTokens {
    pub const DEFAULT_SPATIAL: MotionSchemeToken = MotionSchemeToken::DefaultSpatial;
    pub const FAST_SPATIAL: MotionSchemeToken = MotionSchemeToken::FastSpatial;
    pub const SLOW_SPATIAL: MotionSchemeToken = MotionSchemeToken::SlowSpatial;
    pub const DEFAULT_EFFECTS: MotionSchemeToken = MotionSchemeToken::DefaultEffects;
    pub const FAST_EFFECTS: MotionSchemeToken = MotionSchemeToken::FastEffects;
    pub const SLOW_EFFECTS: MotionSchemeToken = MotionSchemeToken::SlowEffects;
}
impl MotionSchemeToken {
    pub fn resolve(self, tokens: &TokenSet) -> MotionSpec {
        *tokens.motion.spec(match self {
            Self::DefaultSpatial => MotionRole::DefaultSpatial,
            Self::FastSpatial => MotionRole::FastSpatial,
            Self::SlowSpatial => MotionRole::SlowSpatial,
            Self::DefaultEffects => MotionRole::DefaultEffects,
            Self::FastEffects => MotionRole::FastEffects,
            Self::SlowEffects => MotionRole::SlowEffects,
        })
    }
}
impl MotionSchemeKeyTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "MotionSchemeKeyTokens",
            name: "DefaultSpatial",
            value: TokenValue::MotionRole(MotionSchemeKeyTokens::DEFAULT_SPATIAL),
        },
        TokenEntry {
            group: "MotionSchemeKeyTokens",
            name: "FastSpatial",
            value: TokenValue::MotionRole(MotionSchemeKeyTokens::FAST_SPATIAL),
        },
        TokenEntry {
            group: "MotionSchemeKeyTokens",
            name: "SlowSpatial",
            value: TokenValue::MotionRole(MotionSchemeKeyTokens::SLOW_SPATIAL),
        },
        TokenEntry {
            group: "MotionSchemeKeyTokens",
            name: "DefaultEffects",
            value: TokenValue::MotionRole(MotionSchemeKeyTokens::DEFAULT_EFFECTS),
        },
        TokenEntry {
            group: "MotionSchemeKeyTokens",
            name: "FastEffects",
            value: TokenValue::MotionRole(MotionSchemeKeyTokens::FAST_EFFECTS),
        },
        TokenEntry {
            group: "MotionSchemeKeyTokens",
            name: "SlowEffects",
            value: TokenValue::MotionRole(MotionSchemeKeyTokens::SLOW_EFFECTS),
        },
    ];
}
