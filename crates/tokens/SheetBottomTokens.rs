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
pub struct SheetBottomTokens;
impl SheetBottomTokens {
    pub const DOCKED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_LOW;
    pub const DOCKED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE_TOP;
    pub const DOCKED_DRAG_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const DOCKED_DRAG_HANDLE_HEIGHT: Dp = Dp(4.0);
    pub const DOCKED_DRAG_HANDLE_WIDTH: Dp = Dp(32.0);
    pub const DOCKED_MINIMIZED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_NONE;
    pub const DOCKED_MODAL_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const DOCKED_STANDARD_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
}
impl SheetBottomTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "SheetBottomTokens",
            name: "DockedContainerColor",
            value: TokenValue::ColorRole(SheetBottomTokens::DOCKED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "SheetBottomTokens",
            name: "DockedContainerShape",
            value: TokenValue::ShapeRole(SheetBottomTokens::DOCKED_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "SheetBottomTokens",
            name: "DockedDragHandleColor",
            value: TokenValue::ColorRole(SheetBottomTokens::DOCKED_DRAG_HANDLE_COLOR),
        },
        TokenEntry {
            group: "SheetBottomTokens",
            name: "DockedDragHandleHeight",
            value: TokenValue::Dp(SheetBottomTokens::DOCKED_DRAG_HANDLE_HEIGHT),
        },
        TokenEntry {
            group: "SheetBottomTokens",
            name: "DockedDragHandleWidth",
            value: TokenValue::Dp(SheetBottomTokens::DOCKED_DRAG_HANDLE_WIDTH),
        },
        TokenEntry {
            group: "SheetBottomTokens",
            name: "DockedMinimizedContainerShape",
            value: TokenValue::ShapeRole(SheetBottomTokens::DOCKED_MINIMIZED_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "SheetBottomTokens",
            name: "DockedModalContainerElevation",
            value: TokenValue::Dp(SheetBottomTokens::DOCKED_MODAL_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "SheetBottomTokens",
            name: "DockedStandardContainerElevation",
            value: TokenValue::Dp(SheetBottomTokens::DOCKED_STANDARD_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "SheetBottomTokens",
            name: "FocusIndicatorColor",
            value: TokenValue::ColorRole(SheetBottomTokens::FOCUS_INDICATOR_COLOR),
        },
    ];
}
