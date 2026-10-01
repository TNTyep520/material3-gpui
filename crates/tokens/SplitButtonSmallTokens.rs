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
pub struct SplitButtonSmallTokens;
impl SplitButtonSmallTokens {
    pub const BETWEEN_SPACE: Dp = Dp(2.0);
    pub const CONTAINER_HEIGHT: Dp = Dp(40.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const INNER_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_EXTRA_SMALL;
    pub const INNER_HOVERED_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_MEDIUM;
    pub const INNER_PRESSED_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_MEDIUM;
    pub const LEADING_BUTTON_LEADING_SPACE: Dp = Dp(16.0);
    pub const LEADING_BUTTON_TRAILING_SPACE: Dp = Dp(12.0);
    pub const TRAILING_ICON_SIZE: Dp = Dp(22.0);
    pub const TRAILING_INNER_SELECTED_CORNER_CORNER_SIZE_PERCENT: f32 = 50.0;
    pub const TRAILING_BUTTON_LEADING_SPACE: Dp = Dp(13.0);
    pub const TRAILING_BUTTON_TRAILING_SPACE: Dp = Dp(13.0);
}
impl SplitButtonSmallTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "SplitButtonSmallTokens",
            name: "BetweenSpace",
            value: TokenValue::Dp(SplitButtonSmallTokens::BETWEEN_SPACE),
        },
        TokenEntry {
            group: "SplitButtonSmallTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(SplitButtonSmallTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "SplitButtonSmallTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(SplitButtonSmallTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "SplitButtonSmallTokens",
            name: "InnerCornerCornerSize",
            value: TokenValue::Dp(SplitButtonSmallTokens::INNER_CORNER_CORNER_SIZE),
        },
        TokenEntry {
            group: "SplitButtonSmallTokens",
            name: "InnerHoveredCornerCornerSize",
            value: TokenValue::Dp(SplitButtonSmallTokens::INNER_HOVERED_CORNER_CORNER_SIZE),
        },
        TokenEntry {
            group: "SplitButtonSmallTokens",
            name: "InnerPressedCornerCornerSize",
            value: TokenValue::Dp(SplitButtonSmallTokens::INNER_PRESSED_CORNER_CORNER_SIZE),
        },
        TokenEntry {
            group: "SplitButtonSmallTokens",
            name: "LeadingButtonLeadingSpace",
            value: TokenValue::Dp(SplitButtonSmallTokens::LEADING_BUTTON_LEADING_SPACE),
        },
        TokenEntry {
            group: "SplitButtonSmallTokens",
            name: "LeadingButtonTrailingSpace",
            value: TokenValue::Dp(SplitButtonSmallTokens::LEADING_BUTTON_TRAILING_SPACE),
        },
        TokenEntry {
            group: "SplitButtonSmallTokens",
            name: "TrailingIconSize",
            value: TokenValue::Dp(SplitButtonSmallTokens::TRAILING_ICON_SIZE),
        },
        TokenEntry {
            group: "SplitButtonSmallTokens",
            name: "TrailingInnerSelectedCornerCornerSizePercent",
            value: TokenValue::Float(
                SplitButtonSmallTokens::TRAILING_INNER_SELECTED_CORNER_CORNER_SIZE_PERCENT,
            ),
        },
        TokenEntry {
            group: "SplitButtonSmallTokens",
            name: "TrailingButtonLeadingSpace",
            value: TokenValue::Dp(SplitButtonSmallTokens::TRAILING_BUTTON_LEADING_SPACE),
        },
        TokenEntry {
            group: "SplitButtonSmallTokens",
            name: "TrailingButtonTrailingSpace",
            value: TokenValue::Dp(SplitButtonSmallTokens::TRAILING_BUTTON_TRAILING_SPACE),
        },
    ];
}
