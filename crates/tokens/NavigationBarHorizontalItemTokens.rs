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
pub struct NavigationBarHorizontalItemTokens;
impl NavigationBarHorizontalItemTokens {
    pub const ACTIVE_INDICATOR_HEIGHT: Dp = Dp(40.0);
    pub const ACTIVE_INDICATOR_LEADING_SPACE: Dp = Dp(16.0);
    pub const ACTIVE_INDICATOR_TRAILING_SPACE: Dp = Dp(16.0);
    pub const ICON_SIZE: Dp = Dp(24.0);
}
impl NavigationBarHorizontalItemTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "NavigationBarHorizontalItemTokens",
            name: "ActiveIndicatorHeight",
            value: TokenValue::Dp(NavigationBarHorizontalItemTokens::ACTIVE_INDICATOR_HEIGHT),
        },
        TokenEntry {
            group: "NavigationBarHorizontalItemTokens",
            name: "ActiveIndicatorLeadingSpace",
            value: TokenValue::Dp(
                NavigationBarHorizontalItemTokens::ACTIVE_INDICATOR_LEADING_SPACE,
            ),
        },
        TokenEntry {
            group: "NavigationBarHorizontalItemTokens",
            name: "ActiveIndicatorTrailingSpace",
            value: TokenValue::Dp(
                NavigationBarHorizontalItemTokens::ACTIVE_INDICATOR_TRAILING_SPACE,
            ),
        },
        TokenEntry {
            group: "NavigationBarHorizontalItemTokens",
            name: "IconSize",
            value: TokenValue::Dp(NavigationBarHorizontalItemTokens::ICON_SIZE),
        },
    ];
}
