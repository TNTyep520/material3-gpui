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
pub struct SmallIconButtonTokens;
impl SmallIconButtonTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(40.0);
    pub const CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const DEFAULT_LEADING_SPACE: Dp = Dp(8.0);
    pub const DEFAULT_TRAILING_SPACE: Dp = Dp(8.0);
    pub const ICON_SIZE: Dp = Dp(24.0);
    pub const NARROW_LEADING_SPACE: Dp = Dp(4.0);
    pub const NARROW_TRAILING_SPACE: Dp = Dp(4.0);
    pub const OUTLINED_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const PRESSED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const SELECTED_CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const SELECTED_CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const WIDE_LEADING_SPACE: Dp = Dp(14.0);
    pub const WIDE_TRAILING_SPACE: Dp = Dp(14.0);
}
impl SmallIconButtonTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "SmallIconButtonTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(SmallIconButtonTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "SmallIconButtonTokens",
            name: "ContainerShapeRound",
            value: TokenValue::ShapeRole(SmallIconButtonTokens::CONTAINER_SHAPE_ROUND),
        },
        TokenEntry {
            group: "SmallIconButtonTokens",
            name: "ContainerShapeSquare",
            value: TokenValue::ShapeRole(SmallIconButtonTokens::CONTAINER_SHAPE_SQUARE),
        },
        TokenEntry {
            group: "SmallIconButtonTokens",
            name: "DefaultLeadingSpace",
            value: TokenValue::Dp(SmallIconButtonTokens::DEFAULT_LEADING_SPACE),
        },
        TokenEntry {
            group: "SmallIconButtonTokens",
            name: "DefaultTrailingSpace",
            value: TokenValue::Dp(SmallIconButtonTokens::DEFAULT_TRAILING_SPACE),
        },
        TokenEntry {
            group: "SmallIconButtonTokens",
            name: "IconSize",
            value: TokenValue::Dp(SmallIconButtonTokens::ICON_SIZE),
        },
        TokenEntry {
            group: "SmallIconButtonTokens",
            name: "NarrowLeadingSpace",
            value: TokenValue::Dp(SmallIconButtonTokens::NARROW_LEADING_SPACE),
        },
        TokenEntry {
            group: "SmallIconButtonTokens",
            name: "NarrowTrailingSpace",
            value: TokenValue::Dp(SmallIconButtonTokens::NARROW_TRAILING_SPACE),
        },
        TokenEntry {
            group: "SmallIconButtonTokens",
            name: "OutlinedOutlineWidth",
            value: TokenValue::Dp(SmallIconButtonTokens::OUTLINED_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "SmallIconButtonTokens",
            name: "PressedContainerShape",
            value: TokenValue::ShapeRole(SmallIconButtonTokens::PRESSED_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "SmallIconButtonTokens",
            name: "SelectedContainerShapeRound",
            value: TokenValue::ShapeRole(SmallIconButtonTokens::SELECTED_CONTAINER_SHAPE_ROUND),
        },
        TokenEntry {
            group: "SmallIconButtonTokens",
            name: "SelectedContainerShapeSquare",
            value: TokenValue::ShapeRole(SmallIconButtonTokens::SELECTED_CONTAINER_SHAPE_SQUARE),
        },
        TokenEntry {
            group: "SmallIconButtonTokens",
            name: "WideLeadingSpace",
            value: TokenValue::Dp(SmallIconButtonTokens::WIDE_LEADING_SPACE),
        },
        TokenEntry {
            group: "SmallIconButtonTokens",
            name: "WideTrailingSpace",
            value: TokenValue::Dp(SmallIconButtonTokens::WIDE_TRAILING_SPACE),
        },
    ];
}
