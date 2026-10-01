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

use super::{Dp, TokenEntry, TokenValue, TypographyKeyTokens, TypographyToken};

#[derive(Clone, Copy, Debug, Default)]
pub struct NavigationRailVerticalItemTokens;
impl NavigationRailVerticalItemTokens {
    pub const ACTIVE_INDICATOR_HEIGHT: Dp = Dp(32.0);
    pub const ACTIVE_INDICATOR_WIDTH: Dp = Dp(56.0);
    pub const ICON_LABEL_SPACE: Dp = Dp(4.0);
    pub const LEADING_SPACE: Dp = Dp(16.0);
    pub const TRAILING_SPACE: Dp = Dp(16.0);
    pub const LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_MEDIUM;
}
impl NavigationRailVerticalItemTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "NavigationRailVerticalItemTokens",
            name: "ActiveIndicatorHeight",
            value: TokenValue::Dp(NavigationRailVerticalItemTokens::ACTIVE_INDICATOR_HEIGHT),
        },
        TokenEntry {
            group: "NavigationRailVerticalItemTokens",
            name: "ActiveIndicatorWidth",
            value: TokenValue::Dp(NavigationRailVerticalItemTokens::ACTIVE_INDICATOR_WIDTH),
        },
        TokenEntry {
            group: "NavigationRailVerticalItemTokens",
            name: "IconLabelSpace",
            value: TokenValue::Dp(NavigationRailVerticalItemTokens::ICON_LABEL_SPACE),
        },
        TokenEntry {
            group: "NavigationRailVerticalItemTokens",
            name: "LeadingSpace",
            value: TokenValue::Dp(NavigationRailVerticalItemTokens::LEADING_SPACE),
        },
        TokenEntry {
            group: "NavigationRailVerticalItemTokens",
            name: "TrailingSpace",
            value: TokenValue::Dp(NavigationRailVerticalItemTokens::TRAILING_SPACE),
        },
        TokenEntry {
            group: "NavigationRailVerticalItemTokens",
            name: "LabelTextFont",
            value: TokenValue::TypographyRole(NavigationRailVerticalItemTokens::LABEL_TEXT_FONT),
        },
    ];
}
