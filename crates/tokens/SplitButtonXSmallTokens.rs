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
pub struct SplitButtonXSmallTokens;
impl SplitButtonXSmallTokens {
    pub const BETWEEN_SPACE: Dp = Dp(2.0);
    pub const CONTAINER_HEIGHT: Dp = Dp(32.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const INNER_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_EXTRA_SMALL;
    pub const INNER_HOVERED_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_SMALL;
    pub const INNER_PRESSED_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_SMALL;
    pub const LEADING_BUTTON_LEADING_SPACE: Dp = Dp(12.0);
    pub const LEADING_BUTTON_TRAILING_SPACE: Dp = Dp(10.0);
    pub const OUTER_CORNER_CORNER_SIZE_PERCENT: f32 = 50.0;
    pub const TRAILING_ICON_SIZE: Dp = Dp(22.0);
    pub const TRAILING_INNER_SELECTED_CORNER_CORNER_SIZE_PERCENT: f32 = 50.0;
    pub const TRAILING_BUTTON_LEADING_SPACE: Dp = Dp(13.0);
    pub const TRAILING_BUTTON_TRAILING_SPACE: Dp = Dp(13.0);
}
impl SplitButtonXSmallTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "SplitButtonXSmallTokens",
            name: "BetweenSpace",
            value: TokenValue::Dp(SplitButtonXSmallTokens::BETWEEN_SPACE),
        },
        TokenEntry {
            group: "SplitButtonXSmallTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(SplitButtonXSmallTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "SplitButtonXSmallTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(SplitButtonXSmallTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "SplitButtonXSmallTokens",
            name: "InnerCornerCornerSize",
            value: TokenValue::Dp(SplitButtonXSmallTokens::INNER_CORNER_CORNER_SIZE),
        },
        TokenEntry {
            group: "SplitButtonXSmallTokens",
            name: "InnerHoveredCornerCornerSize",
            value: TokenValue::Dp(SplitButtonXSmallTokens::INNER_HOVERED_CORNER_CORNER_SIZE),
        },
        TokenEntry {
            group: "SplitButtonXSmallTokens",
            name: "InnerPressedCornerCornerSize",
            value: TokenValue::Dp(SplitButtonXSmallTokens::INNER_PRESSED_CORNER_CORNER_SIZE),
        },
        TokenEntry {
            group: "SplitButtonXSmallTokens",
            name: "LeadingButtonLeadingSpace",
            value: TokenValue::Dp(SplitButtonXSmallTokens::LEADING_BUTTON_LEADING_SPACE),
        },
        TokenEntry {
            group: "SplitButtonXSmallTokens",
            name: "LeadingButtonTrailingSpace",
            value: TokenValue::Dp(SplitButtonXSmallTokens::LEADING_BUTTON_TRAILING_SPACE),
        },
        TokenEntry {
            group: "SplitButtonXSmallTokens",
            name: "OuterCornerCornerSizePercent",
            value: TokenValue::Float(SplitButtonXSmallTokens::OUTER_CORNER_CORNER_SIZE_PERCENT),
        },
        TokenEntry {
            group: "SplitButtonXSmallTokens",
            name: "TrailingIconSize",
            value: TokenValue::Dp(SplitButtonXSmallTokens::TRAILING_ICON_SIZE),
        },
        TokenEntry {
            group: "SplitButtonXSmallTokens",
            name: "TrailingInnerSelectedCornerCornerSizePercent",
            value: TokenValue::Float(
                SplitButtonXSmallTokens::TRAILING_INNER_SELECTED_CORNER_CORNER_SIZE_PERCENT,
            ),
        },
        TokenEntry {
            group: "SplitButtonXSmallTokens",
            name: "TrailingButtonLeadingSpace",
            value: TokenValue::Dp(SplitButtonXSmallTokens::TRAILING_BUTTON_LEADING_SPACE),
        },
        TokenEntry {
            group: "SplitButtonXSmallTokens",
            name: "TrailingButtonTrailingSpace",
            value: TokenValue::Dp(SplitButtonXSmallTokens::TRAILING_BUTTON_TRAILING_SPACE),
        },
    ];
}
