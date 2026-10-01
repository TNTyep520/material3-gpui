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
pub struct ButtonXSmallTokens;
impl ButtonXSmallTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(32.0);
    pub const CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const ICON_LABEL_SPACE: Dp = Dp(8.0);
    pub const ICON_SIZE: Dp = Dp(20.0);
    pub const LEADING_SPACE: Dp = Dp(16.0);
    pub const OUTLINED_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const PRESSED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const SELECTED_CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const SELECTED_CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const TRAILING_SPACE: Dp = Dp(16.0);
}
impl ButtonXSmallTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "ButtonXSmallTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(ButtonXSmallTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "ButtonXSmallTokens",
            name: "ContainerShapeRound",
            value: TokenValue::ShapeRole(ButtonXSmallTokens::CONTAINER_SHAPE_ROUND),
        },
        TokenEntry {
            group: "ButtonXSmallTokens",
            name: "ContainerShapeSquare",
            value: TokenValue::ShapeRole(ButtonXSmallTokens::CONTAINER_SHAPE_SQUARE),
        },
        TokenEntry {
            group: "ButtonXSmallTokens",
            name: "IconLabelSpace",
            value: TokenValue::Dp(ButtonXSmallTokens::ICON_LABEL_SPACE),
        },
        TokenEntry {
            group: "ButtonXSmallTokens",
            name: "IconSize",
            value: TokenValue::Dp(ButtonXSmallTokens::ICON_SIZE),
        },
        TokenEntry {
            group: "ButtonXSmallTokens",
            name: "LeadingSpace",
            value: TokenValue::Dp(ButtonXSmallTokens::LEADING_SPACE),
        },
        TokenEntry {
            group: "ButtonXSmallTokens",
            name: "OutlinedOutlineWidth",
            value: TokenValue::Dp(ButtonXSmallTokens::OUTLINED_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "ButtonXSmallTokens",
            name: "PressedContainerShape",
            value: TokenValue::ShapeRole(ButtonXSmallTokens::PRESSED_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "ButtonXSmallTokens",
            name: "SelectedContainerShapeRound",
            value: TokenValue::ShapeRole(ButtonXSmallTokens::SELECTED_CONTAINER_SHAPE_ROUND),
        },
        TokenEntry {
            group: "ButtonXSmallTokens",
            name: "SelectedContainerShapeSquare",
            value: TokenValue::ShapeRole(ButtonXSmallTokens::SELECTED_CONTAINER_SHAPE_SQUARE),
        },
        TokenEntry {
            group: "ButtonXSmallTokens",
            name: "TrailingSpace",
            value: TokenValue::Dp(ButtonXSmallTokens::TRAILING_SPACE),
        },
    ];
}
