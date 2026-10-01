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

use super::{Dp, TokenEntry, TokenValue};

#[derive(Clone, Copy, Debug, Default)]
pub struct CircularProgressIndicatorTokens;
impl CircularProgressIndicatorTokens {
    pub const ACTIVE_THICKNESS: Dp = Dp(4.0);
    pub const ACTIVE_WAVE_AMPLITUDE: Dp = Dp(1.6);
    pub const ACTIVE_WAVE_WAVELENGTH: Dp = Dp(15.0);
    pub const SIZE: Dp = Dp(40.0);
    pub const TRACK_ACTIVE_SPACE: Dp = Dp(4.0);
    pub const TRACK_THICKNESS: Dp = Dp(4.0);
    pub const WAVE_SIZE: Dp = Dp(48.0);
}
impl CircularProgressIndicatorTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "CircularProgressIndicatorTokens",
            name: "ActiveThickness",
            value: TokenValue::Dp(CircularProgressIndicatorTokens::ACTIVE_THICKNESS),
        },
        TokenEntry {
            group: "CircularProgressIndicatorTokens",
            name: "ActiveWaveAmplitude",
            value: TokenValue::Dp(CircularProgressIndicatorTokens::ACTIVE_WAVE_AMPLITUDE),
        },
        TokenEntry {
            group: "CircularProgressIndicatorTokens",
            name: "ActiveWaveWavelength",
            value: TokenValue::Dp(CircularProgressIndicatorTokens::ACTIVE_WAVE_WAVELENGTH),
        },
        TokenEntry {
            group: "CircularProgressIndicatorTokens",
            name: "Size",
            value: TokenValue::Dp(CircularProgressIndicatorTokens::SIZE),
        },
        TokenEntry {
            group: "CircularProgressIndicatorTokens",
            name: "TrackActiveSpace",
            value: TokenValue::Dp(CircularProgressIndicatorTokens::TRACK_ACTIVE_SPACE),
        },
        TokenEntry {
            group: "CircularProgressIndicatorTokens",
            name: "TrackThickness",
            value: TokenValue::Dp(CircularProgressIndicatorTokens::TRACK_THICKNESS),
        },
        TokenEntry {
            group: "CircularProgressIndicatorTokens",
            name: "WaveSize",
            value: TokenValue::Dp(CircularProgressIndicatorTokens::WAVE_SIZE),
        },
    ];
}
