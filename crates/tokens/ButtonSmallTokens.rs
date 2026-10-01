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
pub struct ButtonSmallTokens;
impl ButtonSmallTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(40.0);
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
impl ButtonSmallTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "ButtonSmallTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(ButtonSmallTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "ButtonSmallTokens",
            name: "ContainerShapeRound",
            value: TokenValue::ShapeRole(ButtonSmallTokens::CONTAINER_SHAPE_ROUND),
        },
        TokenEntry {
            group: "ButtonSmallTokens",
            name: "ContainerShapeSquare",
            value: TokenValue::ShapeRole(ButtonSmallTokens::CONTAINER_SHAPE_SQUARE),
        },
        TokenEntry {
            group: "ButtonSmallTokens",
            name: "IconLabelSpace",
            value: TokenValue::Dp(ButtonSmallTokens::ICON_LABEL_SPACE),
        },
        TokenEntry {
            group: "ButtonSmallTokens",
            name: "IconSize",
            value: TokenValue::Dp(ButtonSmallTokens::ICON_SIZE),
        },
        TokenEntry {
            group: "ButtonSmallTokens",
            name: "LeadingSpace",
            value: TokenValue::Dp(ButtonSmallTokens::LEADING_SPACE),
        },
        TokenEntry {
            group: "ButtonSmallTokens",
            name: "OutlinedOutlineWidth",
            value: TokenValue::Dp(ButtonSmallTokens::OUTLINED_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "ButtonSmallTokens",
            name: "PressedContainerShape",
            value: TokenValue::ShapeRole(ButtonSmallTokens::PRESSED_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "ButtonSmallTokens",
            name: "SelectedContainerShapeRound",
            value: TokenValue::ShapeRole(ButtonSmallTokens::SELECTED_CONTAINER_SHAPE_ROUND),
        },
        TokenEntry {
            group: "ButtonSmallTokens",
            name: "SelectedContainerShapeSquare",
            value: TokenValue::ShapeRole(ButtonSmallTokens::SELECTED_CONTAINER_SHAPE_SQUARE),
        },
        TokenEntry {
            group: "ButtonSmallTokens",
            name: "TrailingSpace",
            value: TokenValue::Dp(ButtonSmallTokens::TRAILING_SPACE),
        },
    ];
}
