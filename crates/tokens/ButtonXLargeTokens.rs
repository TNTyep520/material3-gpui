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
pub struct ButtonXLargeTokens;
impl ButtonXLargeTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(136.0);
    pub const CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const ICON_LABEL_SPACE: Dp = Dp(16.0);
    pub const ICON_SIZE: Dp = Dp(40.0);
    pub const LEADING_SPACE: Dp = Dp(64.0);
    pub const OUTLINED_OUTLINE_WIDTH: Dp = Dp(3.0);
    pub const PRESSED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const SELECTED_CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const SELECTED_CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const TRAILING_SPACE: Dp = Dp(64.0);
}
impl ButtonXLargeTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "ButtonXLargeTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(ButtonXLargeTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "ButtonXLargeTokens",
            name: "ContainerShapeRound",
            value: TokenValue::ShapeRole(ButtonXLargeTokens::CONTAINER_SHAPE_ROUND),
        },
        TokenEntry {
            group: "ButtonXLargeTokens",
            name: "ContainerShapeSquare",
            value: TokenValue::ShapeRole(ButtonXLargeTokens::CONTAINER_SHAPE_SQUARE),
        },
        TokenEntry {
            group: "ButtonXLargeTokens",
            name: "IconLabelSpace",
            value: TokenValue::Dp(ButtonXLargeTokens::ICON_LABEL_SPACE),
        },
        TokenEntry {
            group: "ButtonXLargeTokens",
            name: "IconSize",
            value: TokenValue::Dp(ButtonXLargeTokens::ICON_SIZE),
        },
        TokenEntry {
            group: "ButtonXLargeTokens",
            name: "LeadingSpace",
            value: TokenValue::Dp(ButtonXLargeTokens::LEADING_SPACE),
        },
        TokenEntry {
            group: "ButtonXLargeTokens",
            name: "OutlinedOutlineWidth",
            value: TokenValue::Dp(ButtonXLargeTokens::OUTLINED_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "ButtonXLargeTokens",
            name: "PressedContainerShape",
            value: TokenValue::ShapeRole(ButtonXLargeTokens::PRESSED_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "ButtonXLargeTokens",
            name: "SelectedContainerShapeRound",
            value: TokenValue::ShapeRole(ButtonXLargeTokens::SELECTED_CONTAINER_SHAPE_ROUND),
        },
        TokenEntry {
            group: "ButtonXLargeTokens",
            name: "SelectedContainerShapeSquare",
            value: TokenValue::ShapeRole(ButtonXLargeTokens::SELECTED_CONTAINER_SHAPE_SQUARE),
        },
        TokenEntry {
            group: "ButtonXLargeTokens",
            name: "TrailingSpace",
            value: TokenValue::Dp(ButtonXLargeTokens::TRAILING_SPACE),
        },
    ];
}
