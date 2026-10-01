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
pub struct XLargeIconButtonTokens;
impl XLargeIconButtonTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(136.0);
    pub const CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const DEFAULT_LEADING_SPACE: Dp = Dp(48.0);
    pub const DEFAULT_TRAILING_SPACE: Dp = Dp(48.0);
    pub const ICON_SIZE: Dp = Dp(40.0);
    pub const NARROW_LEADING_SPACE: Dp = Dp(32.0);
    pub const NARROW_TRAILING_SPACE: Dp = Dp(32.0);
    pub const OUTLINED_OUTLINE_WIDTH: Dp = Dp(3.0);
    pub const PRESSED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const SELECTED_CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const SELECTED_CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const WIDE_LEADING_SPACE: Dp = Dp(72.0);
    pub const WIDE_TRAILING_SPACE: Dp = Dp(72.0);
}
impl XLargeIconButtonTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "XLargeIconButtonTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(XLargeIconButtonTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "XLargeIconButtonTokens",
            name: "ContainerShapeRound",
            value: TokenValue::ShapeRole(XLargeIconButtonTokens::CONTAINER_SHAPE_ROUND),
        },
        TokenEntry {
            group: "XLargeIconButtonTokens",
            name: "ContainerShapeSquare",
            value: TokenValue::ShapeRole(XLargeIconButtonTokens::CONTAINER_SHAPE_SQUARE),
        },
        TokenEntry {
            group: "XLargeIconButtonTokens",
            name: "DefaultLeadingSpace",
            value: TokenValue::Dp(XLargeIconButtonTokens::DEFAULT_LEADING_SPACE),
        },
        TokenEntry {
            group: "XLargeIconButtonTokens",
            name: "DefaultTrailingSpace",
            value: TokenValue::Dp(XLargeIconButtonTokens::DEFAULT_TRAILING_SPACE),
        },
        TokenEntry {
            group: "XLargeIconButtonTokens",
            name: "IconSize",
            value: TokenValue::Dp(XLargeIconButtonTokens::ICON_SIZE),
        },
        TokenEntry {
            group: "XLargeIconButtonTokens",
            name: "NarrowLeadingSpace",
            value: TokenValue::Dp(XLargeIconButtonTokens::NARROW_LEADING_SPACE),
        },
        TokenEntry {
            group: "XLargeIconButtonTokens",
            name: "NarrowTrailingSpace",
            value: TokenValue::Dp(XLargeIconButtonTokens::NARROW_TRAILING_SPACE),
        },
        TokenEntry {
            group: "XLargeIconButtonTokens",
            name: "OutlinedOutlineWidth",
            value: TokenValue::Dp(XLargeIconButtonTokens::OUTLINED_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "XLargeIconButtonTokens",
            name: "PressedContainerShape",
            value: TokenValue::ShapeRole(XLargeIconButtonTokens::PRESSED_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "XLargeIconButtonTokens",
            name: "SelectedContainerShapeRound",
            value: TokenValue::ShapeRole(XLargeIconButtonTokens::SELECTED_CONTAINER_SHAPE_ROUND),
        },
        TokenEntry {
            group: "XLargeIconButtonTokens",
            name: "SelectedContainerShapeSquare",
            value: TokenValue::ShapeRole(XLargeIconButtonTokens::SELECTED_CONTAINER_SHAPE_SQUARE),
        },
        TokenEntry {
            group: "XLargeIconButtonTokens",
            name: "WideLeadingSpace",
            value: TokenValue::Dp(XLargeIconButtonTokens::WIDE_LEADING_SPACE),
        },
        TokenEntry {
            group: "XLargeIconButtonTokens",
            name: "WideTrailingSpace",
            value: TokenValue::Dp(XLargeIconButtonTokens::WIDE_TRAILING_SPACE),
        },
    ];
}
