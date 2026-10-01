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
pub struct ButtonLargeTokens;
impl ButtonLargeTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(96.0);
    pub const CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const ICON_LABEL_SPACE: Dp = Dp(12.0);
    pub const ICON_SIZE: Dp = Dp(32.0);
    pub const LEADING_SPACE: Dp = Dp(48.0);
    pub const OUTLINED_OUTLINE_WIDTH: Dp = Dp(2.0);
    pub const PRESSED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const SELECTED_CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const SELECTED_CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const TRAILING_SPACE: Dp = Dp(48.0);
}
impl ButtonLargeTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "ButtonLargeTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(ButtonLargeTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "ButtonLargeTokens",
            name: "ContainerShapeRound",
            value: TokenValue::ShapeRole(ButtonLargeTokens::CONTAINER_SHAPE_ROUND),
        },
        TokenEntry {
            group: "ButtonLargeTokens",
            name: "ContainerShapeSquare",
            value: TokenValue::ShapeRole(ButtonLargeTokens::CONTAINER_SHAPE_SQUARE),
        },
        TokenEntry {
            group: "ButtonLargeTokens",
            name: "IconLabelSpace",
            value: TokenValue::Dp(ButtonLargeTokens::ICON_LABEL_SPACE),
        },
        TokenEntry {
            group: "ButtonLargeTokens",
            name: "IconSize",
            value: TokenValue::Dp(ButtonLargeTokens::ICON_SIZE),
        },
        TokenEntry {
            group: "ButtonLargeTokens",
            name: "LeadingSpace",
            value: TokenValue::Dp(ButtonLargeTokens::LEADING_SPACE),
        },
        TokenEntry {
            group: "ButtonLargeTokens",
            name: "OutlinedOutlineWidth",
            value: TokenValue::Dp(ButtonLargeTokens::OUTLINED_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "ButtonLargeTokens",
            name: "PressedContainerShape",
            value: TokenValue::ShapeRole(ButtonLargeTokens::PRESSED_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "ButtonLargeTokens",
            name: "SelectedContainerShapeRound",
            value: TokenValue::ShapeRole(ButtonLargeTokens::SELECTED_CONTAINER_SHAPE_ROUND),
        },
        TokenEntry {
            group: "ButtonLargeTokens",
            name: "SelectedContainerShapeSquare",
            value: TokenValue::ShapeRole(ButtonLargeTokens::SELECTED_CONTAINER_SHAPE_SQUARE),
        },
        TokenEntry {
            group: "ButtonLargeTokens",
            name: "TrailingSpace",
            value: TokenValue::Dp(ButtonLargeTokens::TRAILING_SPACE),
        },
    ];
}
