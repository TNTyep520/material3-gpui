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

use super::{
    ColorSchemeKeyTokens, ColorToken, Dp, ElevationTokens, ShapeKeyTokens, ShapeToken, TokenEntry,
    TokenValue,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct DragHandleTokens;
impl DragHandleTokens {
    pub const COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE;
    pub const CONTAINER_WIDTH: Dp = Dp(24.0);
    pub const DRAGGED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DRAGGED_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const DRAGGED_HEIGHT: Dp = Dp(52.0);
    pub const DRAGGED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const DRAGGED_WIDTH: Dp = Dp(12.0);
    pub const ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const HEIGHT: Dp = Dp(48.0);
    pub const PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const PRESSED_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const PRESSED_HEIGHT: Dp = Dp(52.0);
    pub const PRESSED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const PRESSED_WIDTH: Dp = Dp(12.0);
    pub const SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const WIDTH: Dp = Dp(4.0);
}
impl DragHandleTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "DragHandleTokens",
            name: "Color",
            value: TokenValue::ColorRole(DragHandleTokens::COLOR),
        },
        TokenEntry {
            group: "DragHandleTokens",
            name: "ContainerWidth",
            value: TokenValue::Dp(DragHandleTokens::CONTAINER_WIDTH),
        },
        TokenEntry {
            group: "DragHandleTokens",
            name: "DraggedColor",
            value: TokenValue::ColorRole(DragHandleTokens::DRAGGED_COLOR),
        },
        TokenEntry {
            group: "DragHandleTokens",
            name: "DraggedElevation",
            value: TokenValue::Dp(DragHandleTokens::DRAGGED_ELEVATION),
        },
        TokenEntry {
            group: "DragHandleTokens",
            name: "DraggedHeight",
            value: TokenValue::Dp(DragHandleTokens::DRAGGED_HEIGHT),
        },
        TokenEntry {
            group: "DragHandleTokens",
            name: "DraggedShape",
            value: TokenValue::ShapeRole(DragHandleTokens::DRAGGED_SHAPE),
        },
        TokenEntry {
            group: "DragHandleTokens",
            name: "DraggedWidth",
            value: TokenValue::Dp(DragHandleTokens::DRAGGED_WIDTH),
        },
        TokenEntry {
            group: "DragHandleTokens",
            name: "Elevation",
            value: TokenValue::Dp(DragHandleTokens::ELEVATION),
        },
        TokenEntry {
            group: "DragHandleTokens",
            name: "Height",
            value: TokenValue::Dp(DragHandleTokens::HEIGHT),
        },
        TokenEntry {
            group: "DragHandleTokens",
            name: "PressedColor",
            value: TokenValue::ColorRole(DragHandleTokens::PRESSED_COLOR),
        },
        TokenEntry {
            group: "DragHandleTokens",
            name: "PressedElevation",
            value: TokenValue::Dp(DragHandleTokens::PRESSED_ELEVATION),
        },
        TokenEntry {
            group: "DragHandleTokens",
            name: "PressedHeight",
            value: TokenValue::Dp(DragHandleTokens::PRESSED_HEIGHT),
        },
        TokenEntry {
            group: "DragHandleTokens",
            name: "PressedShape",
            value: TokenValue::ShapeRole(DragHandleTokens::PRESSED_SHAPE),
        },
        TokenEntry {
            group: "DragHandleTokens",
            name: "PressedWidth",
            value: TokenValue::Dp(DragHandleTokens::PRESSED_WIDTH),
        },
        TokenEntry {
            group: "DragHandleTokens",
            name: "Shape",
            value: TokenValue::ShapeRole(DragHandleTokens::SHAPE),
        },
        TokenEntry {
            group: "DragHandleTokens",
            name: "Width",
            value: TokenValue::Dp(DragHandleTokens::WIDTH),
        },
    ];
}
