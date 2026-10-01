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

use super::{Dp, ShapeKeyTokens, ShapeToken, TokenEntry, TokenValue};

#[derive(Clone, Copy, Debug, Default)]
pub struct NavigationRailBaselineItemTokens;
impl NavigationRailBaselineItemTokens {
    pub const ACTIVE_INDICATOR_ICON_LABEL_SPACE: Dp = Dp(8.0);
    pub const ACTIVE_INDICATOR_LEADING_SPACE: Dp = Dp(16.0);
    pub const ACTIVE_INDICATOR_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const ACTIVE_INDICATOR_TRAILING_SPACE: Dp = Dp(16.0);
    pub const CONTAINER_HEIGHT: Dp = Dp(64.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_NONE;
    pub const CONTAINER_VERTICAL_SPACE: Dp = Dp(6.0);
    pub const HEADER_SPACE_MINIMUM: Dp = Dp(40.0);
    pub const ICON_SIZE: Dp = Dp(24.0);
}
impl NavigationRailBaselineItemTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "NavigationRailBaselineItemTokens",
            name: "ActiveIndicatorIconLabelSpace",
            value: TokenValue::Dp(
                NavigationRailBaselineItemTokens::ACTIVE_INDICATOR_ICON_LABEL_SPACE,
            ),
        },
        TokenEntry {
            group: "NavigationRailBaselineItemTokens",
            name: "ActiveIndicatorLeadingSpace",
            value: TokenValue::Dp(NavigationRailBaselineItemTokens::ACTIVE_INDICATOR_LEADING_SPACE),
        },
        TokenEntry {
            group: "NavigationRailBaselineItemTokens",
            name: "ActiveIndicatorShape",
            value: TokenValue::ShapeRole(NavigationRailBaselineItemTokens::ACTIVE_INDICATOR_SHAPE),
        },
        TokenEntry {
            group: "NavigationRailBaselineItemTokens",
            name: "ActiveIndicatorTrailingSpace",
            value: TokenValue::Dp(
                NavigationRailBaselineItemTokens::ACTIVE_INDICATOR_TRAILING_SPACE,
            ),
        },
        TokenEntry {
            group: "NavigationRailBaselineItemTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(NavigationRailBaselineItemTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "NavigationRailBaselineItemTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(NavigationRailBaselineItemTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "NavigationRailBaselineItemTokens",
            name: "ContainerVerticalSpace",
            value: TokenValue::Dp(NavigationRailBaselineItemTokens::CONTAINER_VERTICAL_SPACE),
        },
        TokenEntry {
            group: "NavigationRailBaselineItemTokens",
            name: "HeaderSpaceMinimum",
            value: TokenValue::Dp(NavigationRailBaselineItemTokens::HEADER_SPACE_MINIMUM),
        },
        TokenEntry {
            group: "NavigationRailBaselineItemTokens",
            name: "IconSize",
            value: TokenValue::Dp(NavigationRailBaselineItemTokens::ICON_SIZE),
        },
    ];
}
