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
pub struct ElevatedCardTokens;
impl ElevatedCardTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_LOW;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const DISABLED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const DISABLED_CONTAINER_OPACITY: f32 = 0.38;
    pub const DRAGGED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL4;
    pub const FOCUS_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const HOVER_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ICON_SIZE: Dp = Dp(24.0);
    pub const PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
}
impl ElevatedCardTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "ElevatedCardTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(ElevatedCardTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "ElevatedCardTokens",
            name: "ContainerElevation",
            value: TokenValue::Dp(ElevatedCardTokens::CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "ElevatedCardTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(ElevatedCardTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "ElevatedCardTokens",
            name: "DisabledContainerColor",
            value: TokenValue::ColorRole(ElevatedCardTokens::DISABLED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "ElevatedCardTokens",
            name: "DisabledContainerElevation",
            value: TokenValue::Dp(ElevatedCardTokens::DISABLED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "ElevatedCardTokens",
            name: "DisabledContainerOpacity",
            value: TokenValue::Float(ElevatedCardTokens::DISABLED_CONTAINER_OPACITY),
        },
        TokenEntry {
            group: "ElevatedCardTokens",
            name: "DraggedContainerElevation",
            value: TokenValue::Dp(ElevatedCardTokens::DRAGGED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "ElevatedCardTokens",
            name: "FocusContainerElevation",
            value: TokenValue::Dp(ElevatedCardTokens::FOCUS_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "ElevatedCardTokens",
            name: "FocusIndicatorColor",
            value: TokenValue::ColorRole(ElevatedCardTokens::FOCUS_INDICATOR_COLOR),
        },
        TokenEntry {
            group: "ElevatedCardTokens",
            name: "HoverContainerElevation",
            value: TokenValue::Dp(ElevatedCardTokens::HOVER_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "ElevatedCardTokens",
            name: "IconColor",
            value: TokenValue::ColorRole(ElevatedCardTokens::ICON_COLOR),
        },
        TokenEntry {
            group: "ElevatedCardTokens",
            name: "IconSize",
            value: TokenValue::Dp(ElevatedCardTokens::ICON_SIZE),
        },
        TokenEntry {
            group: "ElevatedCardTokens",
            name: "PressedContainerElevation",
            value: TokenValue::Dp(ElevatedCardTokens::PRESSED_CONTAINER_ELEVATION),
        },
    ];
}
