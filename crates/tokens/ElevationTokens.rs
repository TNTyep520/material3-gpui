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
pub struct ElevationTokens;
impl ElevationTokens {
    pub const LEVEL0: Dp = Dp(0.0);
    pub const LEVEL1: Dp = Dp(1.0);
    pub const LEVEL2: Dp = Dp(3.0);
    pub const LEVEL3: Dp = Dp(6.0);
    pub const LEVEL4: Dp = Dp(8.0);
    pub const LEVEL5: Dp = Dp(12.0);
}
impl ElevationTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "ElevationTokens",
            name: "Level0",
            value: TokenValue::Dp(ElevationTokens::LEVEL0),
        },
        TokenEntry {
            group: "ElevationTokens",
            name: "Level1",
            value: TokenValue::Dp(ElevationTokens::LEVEL1),
        },
        TokenEntry {
            group: "ElevationTokens",
            name: "Level2",
            value: TokenValue::Dp(ElevationTokens::LEVEL2),
        },
        TokenEntry {
            group: "ElevationTokens",
            name: "Level3",
            value: TokenValue::Dp(ElevationTokens::LEVEL3),
        },
        TokenEntry {
            group: "ElevationTokens",
            name: "Level4",
            value: TokenValue::Dp(ElevationTokens::LEVEL4),
        },
        TokenEntry {
            group: "ElevationTokens",
            name: "Level5",
            value: TokenValue::Dp(ElevationTokens::LEVEL5),
        },
    ];
}
