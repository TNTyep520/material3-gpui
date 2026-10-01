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
pub struct ButtonMediumTokens;
impl ButtonMediumTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(56.0);
    pub const CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const ICON_LABEL_SPACE: Dp = Dp(8.0);
    pub const ICON_SIZE: Dp = Dp(24.0);
    pub const LEADING_SPACE: Dp = Dp(24.0);
    pub const OUTLINED_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const PRESSED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const SELECTED_CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const SELECTED_CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const TRAILING_SPACE: Dp = Dp(24.0);
}
impl ButtonMediumTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "ButtonMediumTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(ButtonMediumTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "ButtonMediumTokens",
            name: "ContainerShapeRound",
            value: TokenValue::ShapeRole(ButtonMediumTokens::CONTAINER_SHAPE_ROUND),
        },
        TokenEntry {
            group: "ButtonMediumTokens",
            name: "ContainerShapeSquare",
            value: TokenValue::ShapeRole(ButtonMediumTokens::CONTAINER_SHAPE_SQUARE),
        },
        TokenEntry {
            group: "ButtonMediumTokens",
            name: "IconLabelSpace",
            value: TokenValue::Dp(ButtonMediumTokens::ICON_LABEL_SPACE),
        },
        TokenEntry {
            group: "ButtonMediumTokens",
            name: "IconSize",
            value: TokenValue::Dp(ButtonMediumTokens::ICON_SIZE),
        },
        TokenEntry {
            group: "ButtonMediumTokens",
            name: "LeadingSpace",
            value: TokenValue::Dp(ButtonMediumTokens::LEADING_SPACE),
        },
        TokenEntry {
            group: "ButtonMediumTokens",
            name: "OutlinedOutlineWidth",
            value: TokenValue::Dp(ButtonMediumTokens::OUTLINED_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "ButtonMediumTokens",
            name: "PressedContainerShape",
            value: TokenValue::ShapeRole(ButtonMediumTokens::PRESSED_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "ButtonMediumTokens",
            name: "SelectedContainerShapeRound",
            value: TokenValue::ShapeRole(ButtonMediumTokens::SELECTED_CONTAINER_SHAPE_ROUND),
        },
        TokenEntry {
            group: "ButtonMediumTokens",
            name: "SelectedContainerShapeSquare",
            value: TokenValue::ShapeRole(ButtonMediumTokens::SELECTED_CONTAINER_SHAPE_SQUARE),
        },
        TokenEntry {
            group: "ButtonMediumTokens",
            name: "TrailingSpace",
            value: TokenValue::Dp(ButtonMediumTokens::TRAILING_SPACE),
        },
    ];
}
