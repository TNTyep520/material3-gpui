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
pub struct LargeIconButtonTokens;
impl LargeIconButtonTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(96.0);
    pub const CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const ICON_SIZE: Dp = Dp(32.0);
    pub const NARROW_LEADING_SPACE: Dp = Dp(16.0);
    pub const NARROW_TRAILING_SPACE: Dp = Dp(16.0);
    pub const OUTLINED_OUTLINE_WIDTH: Dp = Dp(2.0);
    pub const PRESSED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const SELECTED_CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const SELECTED_CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const UNIFORM_LEADING_SPACE: Dp = Dp(32.0);
    pub const UNIFORM_TRAILING_SPACE: Dp = Dp(32.0);
    pub const WIDE_LEADING_SPACE: Dp = Dp(48.0);
    pub const WIDE_TRAILING_SPACE: Dp = Dp(48.0);
}
impl LargeIconButtonTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "LargeIconButtonTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(LargeIconButtonTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "LargeIconButtonTokens",
            name: "ContainerShapeRound",
            value: TokenValue::ShapeRole(LargeIconButtonTokens::CONTAINER_SHAPE_ROUND),
        },
        TokenEntry {
            group: "LargeIconButtonTokens",
            name: "ContainerShapeSquare",
            value: TokenValue::ShapeRole(LargeIconButtonTokens::CONTAINER_SHAPE_SQUARE),
        },
        TokenEntry {
            group: "LargeIconButtonTokens",
            name: "IconSize",
            value: TokenValue::Dp(LargeIconButtonTokens::ICON_SIZE),
        },
        TokenEntry {
            group: "LargeIconButtonTokens",
            name: "NarrowLeadingSpace",
            value: TokenValue::Dp(LargeIconButtonTokens::NARROW_LEADING_SPACE),
        },
        TokenEntry {
            group: "LargeIconButtonTokens",
            name: "NarrowTrailingSpace",
            value: TokenValue::Dp(LargeIconButtonTokens::NARROW_TRAILING_SPACE),
        },
        TokenEntry {
            group: "LargeIconButtonTokens",
            name: "OutlinedOutlineWidth",
            value: TokenValue::Dp(LargeIconButtonTokens::OUTLINED_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "LargeIconButtonTokens",
            name: "PressedContainerShape",
            value: TokenValue::ShapeRole(LargeIconButtonTokens::PRESSED_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "LargeIconButtonTokens",
            name: "SelectedContainerShapeRound",
            value: TokenValue::ShapeRole(LargeIconButtonTokens::SELECTED_CONTAINER_SHAPE_ROUND),
        },
        TokenEntry {
            group: "LargeIconButtonTokens",
            name: "SelectedContainerShapeSquare",
            value: TokenValue::ShapeRole(LargeIconButtonTokens::SELECTED_CONTAINER_SHAPE_SQUARE),
        },
        TokenEntry {
            group: "LargeIconButtonTokens",
            name: "UniformLeadingSpace",
            value: TokenValue::Dp(LargeIconButtonTokens::UNIFORM_LEADING_SPACE),
        },
        TokenEntry {
            group: "LargeIconButtonTokens",
            name: "UniformTrailingSpace",
            value: TokenValue::Dp(LargeIconButtonTokens::UNIFORM_TRAILING_SPACE),
        },
        TokenEntry {
            group: "LargeIconButtonTokens",
            name: "WideLeadingSpace",
            value: TokenValue::Dp(LargeIconButtonTokens::WIDE_LEADING_SPACE),
        },
        TokenEntry {
            group: "LargeIconButtonTokens",
            name: "WideTrailingSpace",
            value: TokenValue::Dp(LargeIconButtonTokens::WIDE_TRAILING_SPACE),
        },
    ];
}
