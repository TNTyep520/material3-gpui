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
pub struct XSmallIconButtonTokens;
impl XSmallIconButtonTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(32.0);
    pub const CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const DEFAULT_LEADING_SPACE: Dp = Dp(6.0);
    pub const DEFAULT_TRAILING_SPACE: Dp = Dp(6.0);
    pub const ICON_SIZE: Dp = Dp(20.0);
    pub const NARROW_LEADING_SPACE: Dp = Dp(4.0);
    pub const NARROW_TRAILING_SPACE: Dp = Dp(4.0);
    pub const OUTLINED_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const PRESSED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const SELECTED_CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const SELECTED_CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const WIDE_LEADING_SPACE: Dp = Dp(10.0);
    pub const WIDE_TRAILING_SPACE: Dp = Dp(10.0);
}
impl XSmallIconButtonTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "XSmallIconButtonTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(XSmallIconButtonTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "XSmallIconButtonTokens",
            name: "ContainerShapeRound",
            value: TokenValue::ShapeRole(XSmallIconButtonTokens::CONTAINER_SHAPE_ROUND),
        },
        TokenEntry {
            group: "XSmallIconButtonTokens",
            name: "ContainerShapeSquare",
            value: TokenValue::ShapeRole(XSmallIconButtonTokens::CONTAINER_SHAPE_SQUARE),
        },
        TokenEntry {
            group: "XSmallIconButtonTokens",
            name: "DefaultLeadingSpace",
            value: TokenValue::Dp(XSmallIconButtonTokens::DEFAULT_LEADING_SPACE),
        },
        TokenEntry {
            group: "XSmallIconButtonTokens",
            name: "DefaultTrailingSpace",
            value: TokenValue::Dp(XSmallIconButtonTokens::DEFAULT_TRAILING_SPACE),
        },
        TokenEntry {
            group: "XSmallIconButtonTokens",
            name: "IconSize",
            value: TokenValue::Dp(XSmallIconButtonTokens::ICON_SIZE),
        },
        TokenEntry {
            group: "XSmallIconButtonTokens",
            name: "NarrowLeadingSpace",
            value: TokenValue::Dp(XSmallIconButtonTokens::NARROW_LEADING_SPACE),
        },
        TokenEntry {
            group: "XSmallIconButtonTokens",
            name: "NarrowTrailingSpace",
            value: TokenValue::Dp(XSmallIconButtonTokens::NARROW_TRAILING_SPACE),
        },
        TokenEntry {
            group: "XSmallIconButtonTokens",
            name: "OutlinedOutlineWidth",
            value: TokenValue::Dp(XSmallIconButtonTokens::OUTLINED_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "XSmallIconButtonTokens",
            name: "PressedContainerShape",
            value: TokenValue::ShapeRole(XSmallIconButtonTokens::PRESSED_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "XSmallIconButtonTokens",
            name: "SelectedContainerShapeRound",
            value: TokenValue::ShapeRole(XSmallIconButtonTokens::SELECTED_CONTAINER_SHAPE_ROUND),
        },
        TokenEntry {
            group: "XSmallIconButtonTokens",
            name: "SelectedContainerShapeSquare",
            value: TokenValue::ShapeRole(XSmallIconButtonTokens::SELECTED_CONTAINER_SHAPE_SQUARE),
        },
        TokenEntry {
            group: "XSmallIconButtonTokens",
            name: "WideLeadingSpace",
            value: TokenValue::Dp(XSmallIconButtonTokens::WIDE_LEADING_SPACE),
        },
        TokenEntry {
            group: "XSmallIconButtonTokens",
            name: "WideTrailingSpace",
            value: TokenValue::Dp(XSmallIconButtonTokens::WIDE_TRAILING_SPACE),
        },
    ];
}
