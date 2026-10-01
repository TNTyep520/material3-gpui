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

use super::{ColorSchemeKeyTokens, ColorToken, Dp, ElevationTokens, TokenEntry, TokenValue};

#[derive(Clone, Copy, Debug, Default)]
pub struct FabSecondaryContainerTokens;
impl FabSecondaryContainerTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const FOCUSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const HOVERED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL4;
    pub const HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
}
impl FabSecondaryContainerTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "FabSecondaryContainerTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(FabSecondaryContainerTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "FabSecondaryContainerTokens",
            name: "ContainerElevation",
            value: TokenValue::Dp(FabSecondaryContainerTokens::CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FabSecondaryContainerTokens",
            name: "FocusedContainerElevation",
            value: TokenValue::Dp(FabSecondaryContainerTokens::FOCUSED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FabSecondaryContainerTokens",
            name: "FocusedIconColor",
            value: TokenValue::ColorRole(FabSecondaryContainerTokens::FOCUSED_ICON_COLOR),
        },
        TokenEntry {
            group: "FabSecondaryContainerTokens",
            name: "HoveredContainerElevation",
            value: TokenValue::Dp(FabSecondaryContainerTokens::HOVERED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FabSecondaryContainerTokens",
            name: "HoveredIconColor",
            value: TokenValue::ColorRole(FabSecondaryContainerTokens::HOVERED_ICON_COLOR),
        },
        TokenEntry {
            group: "FabSecondaryContainerTokens",
            name: "IconColor",
            value: TokenValue::ColorRole(FabSecondaryContainerTokens::ICON_COLOR),
        },
        TokenEntry {
            group: "FabSecondaryContainerTokens",
            name: "PressedContainerElevation",
            value: TokenValue::Dp(FabSecondaryContainerTokens::PRESSED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FabSecondaryContainerTokens",
            name: "PressedIconColor",
            value: TokenValue::ColorRole(FabSecondaryContainerTokens::PRESSED_ICON_COLOR),
        },
    ];
}
