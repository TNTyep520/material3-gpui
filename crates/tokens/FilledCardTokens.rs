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
pub struct FilledCardTokens;
impl FilledCardTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_VARIANT;
    pub const DISABLED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const DISABLED_CONTAINER_OPACITY: f32 = 0.38;
    pub const DRAGGED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const FOCUS_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const HOVER_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ICON_SIZE: Dp = Dp(24.0);
    pub const PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
}
impl FilledCardTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "FilledCardTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(FilledCardTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "FilledCardTokens",
            name: "ContainerElevation",
            value: TokenValue::Dp(FilledCardTokens::CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FilledCardTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(FilledCardTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "FilledCardTokens",
            name: "DisabledContainerColor",
            value: TokenValue::ColorRole(FilledCardTokens::DISABLED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "FilledCardTokens",
            name: "DisabledContainerElevation",
            value: TokenValue::Dp(FilledCardTokens::DISABLED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FilledCardTokens",
            name: "DisabledContainerOpacity",
            value: TokenValue::Float(FilledCardTokens::DISABLED_CONTAINER_OPACITY),
        },
        TokenEntry {
            group: "FilledCardTokens",
            name: "DraggedContainerElevation",
            value: TokenValue::Dp(FilledCardTokens::DRAGGED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FilledCardTokens",
            name: "FocusContainerElevation",
            value: TokenValue::Dp(FilledCardTokens::FOCUS_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FilledCardTokens",
            name: "FocusIndicatorColor",
            value: TokenValue::ColorRole(FilledCardTokens::FOCUS_INDICATOR_COLOR),
        },
        TokenEntry {
            group: "FilledCardTokens",
            name: "HoverContainerElevation",
            value: TokenValue::Dp(FilledCardTokens::HOVER_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FilledCardTokens",
            name: "IconColor",
            value: TokenValue::ColorRole(FilledCardTokens::ICON_COLOR),
        },
        TokenEntry {
            group: "FilledCardTokens",
            name: "IconSize",
            value: TokenValue::Dp(FilledCardTokens::ICON_SIZE),
        },
        TokenEntry {
            group: "FilledCardTokens",
            name: "PressedContainerElevation",
            value: TokenValue::Dp(FilledCardTokens::PRESSED_CONTAINER_ELEVATION),
        },
    ];
}
