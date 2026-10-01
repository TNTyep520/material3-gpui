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
pub struct SplitButtonLargeTokens;
impl SplitButtonLargeTokens {
    pub const BETWEEN_SPACE: Dp = Dp(2.0);
    pub const CONTAINER_HEIGHT: Dp = Dp(96.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const INNER_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_SMALL;
    pub const INNER_HOVERED_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_LARGE_INCREASED;
    pub const INNER_PRESSED_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_LARGE_INCREASED;
    pub const LEADING_BUTTON_LEADING_SPACE: Dp = Dp(48.0);
    pub const LEADING_BUTTON_TRAILING_SPACE: Dp = Dp(48.0);
    pub const TRAILING_ICON_SIZE: Dp = Dp(38.0);
    pub const TRAILING_INNER_SELECTED_CORNER_CORNER_SIZE_PERCENT: f32 = 50.0;
    pub const TRAILING_BUTTON_LEADING_SPACE: Dp = Dp(29.0);
    pub const TRAILING_BUTTON_TRAILING_SPACE: Dp = Dp(29.0);
}
impl SplitButtonLargeTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "SplitButtonLargeTokens",
            name: "BetweenSpace",
            value: TokenValue::Dp(SplitButtonLargeTokens::BETWEEN_SPACE),
        },
        TokenEntry {
            group: "SplitButtonLargeTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(SplitButtonLargeTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "SplitButtonLargeTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(SplitButtonLargeTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "SplitButtonLargeTokens",
            name: "InnerCornerCornerSize",
            value: TokenValue::Dp(SplitButtonLargeTokens::INNER_CORNER_CORNER_SIZE),
        },
        TokenEntry {
            group: "SplitButtonLargeTokens",
            name: "InnerHoveredCornerCornerSize",
            value: TokenValue::Dp(SplitButtonLargeTokens::INNER_HOVERED_CORNER_CORNER_SIZE),
        },
        TokenEntry {
            group: "SplitButtonLargeTokens",
            name: "InnerPressedCornerCornerSize",
            value: TokenValue::Dp(SplitButtonLargeTokens::INNER_PRESSED_CORNER_CORNER_SIZE),
        },
        TokenEntry {
            group: "SplitButtonLargeTokens",
            name: "LeadingButtonLeadingSpace",
            value: TokenValue::Dp(SplitButtonLargeTokens::LEADING_BUTTON_LEADING_SPACE),
        },
        TokenEntry {
            group: "SplitButtonLargeTokens",
            name: "LeadingButtonTrailingSpace",
            value: TokenValue::Dp(SplitButtonLargeTokens::LEADING_BUTTON_TRAILING_SPACE),
        },
        TokenEntry {
            group: "SplitButtonLargeTokens",
            name: "TrailingIconSize",
            value: TokenValue::Dp(SplitButtonLargeTokens::TRAILING_ICON_SIZE),
        },
        TokenEntry {
            group: "SplitButtonLargeTokens",
            name: "TrailingInnerSelectedCornerCornerSizePercent",
            value: TokenValue::Float(
                SplitButtonLargeTokens::TRAILING_INNER_SELECTED_CORNER_CORNER_SIZE_PERCENT,
            ),
        },
        TokenEntry {
            group: "SplitButtonLargeTokens",
            name: "TrailingButtonLeadingSpace",
            value: TokenValue::Dp(SplitButtonLargeTokens::TRAILING_BUTTON_LEADING_SPACE),
        },
        TokenEntry {
            group: "SplitButtonLargeTokens",
            name: "TrailingButtonTrailingSpace",
            value: TokenValue::Dp(SplitButtonLargeTokens::TRAILING_BUTTON_TRAILING_SPACE),
        },
    ];
}
