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
pub struct MediumIconButtonTokens;
impl MediumIconButtonTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(56.0);
    pub const CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const DEFAULT_LEADING_SPACE: Dp = Dp(16.0);
    pub const DEFAULT_TRAILING_SPACE: Dp = Dp(16.0);
    pub const ICON_SIZE: Dp = Dp(24.0);
    pub const NARROW_LEADING_SPACE: Dp = Dp(12.0);
    pub const NARROW_TRAILING_SPACE: Dp = Dp(12.0);
    pub const OUTLINED_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const PRESSED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const SELECTED_CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const SELECTED_CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const WIDE_LEADING_SPACE: Dp = Dp(24.0);
    pub const WIDE_TRAILING_SPACE: Dp = Dp(24.0);
}
impl MediumIconButtonTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "MediumIconButtonTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(MediumIconButtonTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "MediumIconButtonTokens",
            name: "ContainerShapeRound",
            value: TokenValue::ShapeRole(MediumIconButtonTokens::CONTAINER_SHAPE_ROUND),
        },
        TokenEntry {
            group: "MediumIconButtonTokens",
            name: "ContainerShapeSquare",
            value: TokenValue::ShapeRole(MediumIconButtonTokens::CONTAINER_SHAPE_SQUARE),
        },
        TokenEntry {
            group: "MediumIconButtonTokens",
            name: "DefaultLeadingSpace",
            value: TokenValue::Dp(MediumIconButtonTokens::DEFAULT_LEADING_SPACE),
        },
        TokenEntry {
            group: "MediumIconButtonTokens",
            name: "DefaultTrailingSpace",
            value: TokenValue::Dp(MediumIconButtonTokens::DEFAULT_TRAILING_SPACE),
        },
        TokenEntry {
            group: "MediumIconButtonTokens",
            name: "IconSize",
            value: TokenValue::Dp(MediumIconButtonTokens::ICON_SIZE),
        },
        TokenEntry {
            group: "MediumIconButtonTokens",
            name: "NarrowLeadingSpace",
            value: TokenValue::Dp(MediumIconButtonTokens::NARROW_LEADING_SPACE),
        },
        TokenEntry {
            group: "MediumIconButtonTokens",
            name: "NarrowTrailingSpace",
            value: TokenValue::Dp(MediumIconButtonTokens::NARROW_TRAILING_SPACE),
        },
        TokenEntry {
            group: "MediumIconButtonTokens",
            name: "OutlinedOutlineWidth",
            value: TokenValue::Dp(MediumIconButtonTokens::OUTLINED_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "MediumIconButtonTokens",
            name: "PressedContainerShape",
            value: TokenValue::ShapeRole(MediumIconButtonTokens::PRESSED_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "MediumIconButtonTokens",
            name: "SelectedContainerShapeRound",
            value: TokenValue::ShapeRole(MediumIconButtonTokens::SELECTED_CONTAINER_SHAPE_ROUND),
        },
        TokenEntry {
            group: "MediumIconButtonTokens",
            name: "SelectedContainerShapeSquare",
            value: TokenValue::ShapeRole(MediumIconButtonTokens::SELECTED_CONTAINER_SHAPE_SQUARE),
        },
        TokenEntry {
            group: "MediumIconButtonTokens",
            name: "WideLeadingSpace",
            value: TokenValue::Dp(MediumIconButtonTokens::WIDE_LEADING_SPACE),
        },
        TokenEntry {
            group: "MediumIconButtonTokens",
            name: "WideTrailingSpace",
            value: TokenValue::Dp(MediumIconButtonTokens::WIDE_TRAILING_SPACE),
        },
    ];
}
