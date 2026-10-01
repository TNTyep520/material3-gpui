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
pub struct SplitButtonMediumTokens;
impl SplitButtonMediumTokens {
    pub const BETWEEN_SPACE: Dp = Dp(2.0);
    pub const CONTAINER_HEIGHT: Dp = Dp(56.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const INNER_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_EXTRA_SMALL;
    pub const INNER_HOVERED_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_MEDIUM;
    pub const INNER_PRESSED_CORNER_CORNER_SIZE: Dp = ShapeTokens::CORNER_VALUE_MEDIUM;
    pub const LEADING_BUTTON_LEADING_SPACE: Dp = Dp(24.0);
    pub const LEADING_BUTTON_TRAILING_SPACE: Dp = Dp(24.0);
    pub const TRAILING_ICON_SIZE: Dp = Dp(26.0);
    pub const TRAILING_INNER_SELECTED_CORNER_CORNER_SIZE_PERCENT: f32 = 50.0;
    pub const TRAILING_BUTTON_LEADING_SPACE: Dp = Dp(15.0);
    pub const TRAILING_BUTTON_TRAILING_SPACE: Dp = Dp(15.0);
}
impl SplitButtonMediumTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "SplitButtonMediumTokens",
            name: "BetweenSpace",
            value: TokenValue::Dp(SplitButtonMediumTokens::BETWEEN_SPACE),
        },
        TokenEntry {
            group: "SplitButtonMediumTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(SplitButtonMediumTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "SplitButtonMediumTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(SplitButtonMediumTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "SplitButtonMediumTokens",
            name: "InnerCornerCornerSize",
            value: TokenValue::Dp(SplitButtonMediumTokens::INNER_CORNER_CORNER_SIZE),
        },
        TokenEntry {
            group: "SplitButtonMediumTokens",
            name: "InnerHoveredCornerCornerSize",
            value: TokenValue::Dp(SplitButtonMediumTokens::INNER_HOVERED_CORNER_CORNER_SIZE),
        },
        TokenEntry {
            group: "SplitButtonMediumTokens",
            name: "InnerPressedCornerCornerSize",
            value: TokenValue::Dp(SplitButtonMediumTokens::INNER_PRESSED_CORNER_CORNER_SIZE),
        },
        TokenEntry {
            group: "SplitButtonMediumTokens",
            name: "LeadingButtonLeadingSpace",
            value: TokenValue::Dp(SplitButtonMediumTokens::LEADING_BUTTON_LEADING_SPACE),
        },
        TokenEntry {
            group: "SplitButtonMediumTokens",
            name: "LeadingButtonTrailingSpace",
            value: TokenValue::Dp(SplitButtonMediumTokens::LEADING_BUTTON_TRAILING_SPACE),
        },
        TokenEntry {
            group: "SplitButtonMediumTokens",
            name: "TrailingIconSize",
            value: TokenValue::Dp(SplitButtonMediumTokens::TRAILING_ICON_SIZE),
        },
        TokenEntry {
            group: "SplitButtonMediumTokens",
            name: "TrailingInnerSelectedCornerCornerSizePercent",
            value: TokenValue::Float(
                SplitButtonMediumTokens::TRAILING_INNER_SELECTED_CORNER_CORNER_SIZE_PERCENT,
            ),
        },
        TokenEntry {
            group: "SplitButtonMediumTokens",
            name: "TrailingButtonLeadingSpace",
            value: TokenValue::Dp(SplitButtonMediumTokens::TRAILING_BUTTON_LEADING_SPACE),
        },
        TokenEntry {
            group: "SplitButtonMediumTokens",
            name: "TrailingButtonTrailingSpace",
            value: TokenValue::Dp(SplitButtonMediumTokens::TRAILING_BUTTON_TRAILING_SPACE),
        },
    ];
}
