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

use super::{Dp, ShapeKeyTokens, ShapeToken, ShapeTokens, TokenEntry, TokenValue};

#[derive(Clone, Copy, Debug, Default)]
pub struct SplitButtonXLargeTokens;
impl SplitButtonXLargeTokens {
    pub const BETWEEN_SPACE: Dp = Dp(2.0);
    pub const CONTAINER_HEIGHT: Dp = Dp(136.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const INNER_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_MEDIUM;
    pub const INNER_HOVERED_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_LARGE_INCREASED;
    pub const INNER_PRESSED_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_LARGE_INCREASED;
    pub const LEADING_BUTTON_LEADING_SPACE: Dp = Dp(64.0);
    pub const LEADING_BUTTON_TRAILING_SPACE: Dp = Dp(64.0);
    pub const TRAILING_ICON_SIZE: Dp = Dp(50.0);
    pub const TRAILING_INNER_SELECTED_CORNER_CORNER_SIZE_PERCENT: f32 = 50.0;
    pub const TRAILING_BUTTON_LEADING_SPACE: Dp = Dp(43.0);
    pub const TRAILING_BUTTON_TRAILING_SPACE: Dp = Dp(43.0);
}
impl SplitButtonXLargeTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "SplitButtonXLargeTokens",
            name: "BetweenSpace",
            value: TokenValue::Dp(SplitButtonXLargeTokens::BETWEEN_SPACE),
        },
        TokenEntry {
            group: "SplitButtonXLargeTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(SplitButtonXLargeTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "SplitButtonXLargeTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(SplitButtonXLargeTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "SplitButtonXLargeTokens",
            name: "InnerCornerCornerSize",
            value: TokenValue::Dp(SplitButtonXLargeTokens::INNER_CORNER_CORNER_SIZE),
        },
        TokenEntry {
            group: "SplitButtonXLargeTokens",
            name: "InnerHoveredCornerCornerSize",
            value: TokenValue::Dp(SplitButtonXLargeTokens::INNER_HOVERED_CORNER_CORNER_SIZE),
        },
        TokenEntry {
            group: "SplitButtonXLargeTokens",
            name: "InnerPressedCornerCornerSize",
            value: TokenValue::Dp(SplitButtonXLargeTokens::INNER_PRESSED_CORNER_CORNER_SIZE),
        },
        TokenEntry {
            group: "SplitButtonXLargeTokens",
            name: "LeadingButtonLeadingSpace",
            value: TokenValue::Dp(SplitButtonXLargeTokens::LEADING_BUTTON_LEADING_SPACE),
        },
        TokenEntry {
            group: "SplitButtonXLargeTokens",
            name: "LeadingButtonTrailingSpace",
            value: TokenValue::Dp(SplitButtonXLargeTokens::LEADING_BUTTON_TRAILING_SPACE),
        },
        TokenEntry {
            group: "SplitButtonXLargeTokens",
            name: "TrailingIconSize",
            value: TokenValue::Dp(SplitButtonXLargeTokens::TRAILING_ICON_SIZE),
        },
        TokenEntry {
            group: "SplitButtonXLargeTokens",
            name: "TrailingInnerSelectedCornerCornerSizePercent",
            value: TokenValue::Float(
                SplitButtonXLargeTokens::TRAILING_INNER_SELECTED_CORNER_CORNER_SIZE_PERCENT,
            ),
        },
        TokenEntry {
            group: "SplitButtonXLargeTokens",
            name: "TrailingButtonLeadingSpace",
            value: TokenValue::Dp(SplitButtonXLargeTokens::TRAILING_BUTTON_LEADING_SPACE),
        },
        TokenEntry {
            group: "SplitButtonXLargeTokens",
            name: "TrailingButtonTrailingSpace",
            value: TokenValue::Dp(SplitButtonXLargeTokens::TRAILING_BUTTON_TRAILING_SPACE),
        },
    ];
}
