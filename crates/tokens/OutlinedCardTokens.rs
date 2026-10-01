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
pub struct OutlinedCardTokens;
impl OutlinedCardTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const DISABLED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const DISABLED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE;
    pub const DISABLED_OUTLINE_OPACITY: f32 = 0.12;
    pub const DRAGGED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const DRAGGED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const FOCUS_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const FOCUS_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const HOVER_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const HOVER_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ICON_SIZE: Dp = Dp(24.0);
    pub const OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const PRESSED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
}
impl OutlinedCardTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "OutlinedCardTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(OutlinedCardTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "OutlinedCardTokens",
            name: "ContainerElevation",
            value: TokenValue::Dp(OutlinedCardTokens::CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "OutlinedCardTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(OutlinedCardTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "OutlinedCardTokens",
            name: "DisabledContainerElevation",
            value: TokenValue::Dp(OutlinedCardTokens::DISABLED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "OutlinedCardTokens",
            name: "DisabledOutlineColor",
            value: TokenValue::ColorRole(OutlinedCardTokens::DISABLED_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedCardTokens",
            name: "DisabledOutlineOpacity",
            value: TokenValue::Float(OutlinedCardTokens::DISABLED_OUTLINE_OPACITY),
        },
        TokenEntry {
            group: "OutlinedCardTokens",
            name: "DraggedContainerElevation",
            value: TokenValue::Dp(OutlinedCardTokens::DRAGGED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "OutlinedCardTokens",
            name: "DraggedOutlineColor",
            value: TokenValue::ColorRole(OutlinedCardTokens::DRAGGED_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedCardTokens",
            name: "FocusContainerElevation",
            value: TokenValue::Dp(OutlinedCardTokens::FOCUS_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "OutlinedCardTokens",
            name: "FocusOutlineColor",
            value: TokenValue::ColorRole(OutlinedCardTokens::FOCUS_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedCardTokens",
            name: "HoverContainerElevation",
            value: TokenValue::Dp(OutlinedCardTokens::HOVER_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "OutlinedCardTokens",
            name: "HoverOutlineColor",
            value: TokenValue::ColorRole(OutlinedCardTokens::HOVER_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedCardTokens",
            name: "IconColor",
            value: TokenValue::ColorRole(OutlinedCardTokens::ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedCardTokens",
            name: "IconSize",
            value: TokenValue::Dp(OutlinedCardTokens::ICON_SIZE),
        },
        TokenEntry {
            group: "OutlinedCardTokens",
            name: "OutlineColor",
            value: TokenValue::ColorRole(OutlinedCardTokens::OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedCardTokens",
            name: "OutlineWidth",
            value: TokenValue::Dp(OutlinedCardTokens::OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "OutlinedCardTokens",
            name: "PressedContainerElevation",
            value: TokenValue::Dp(OutlinedCardTokens::PRESSED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "OutlinedCardTokens",
            name: "PressedOutlineColor",
            value: TokenValue::ColorRole(OutlinedCardTokens::PRESSED_OUTLINE_COLOR),
        },
    ];
}
