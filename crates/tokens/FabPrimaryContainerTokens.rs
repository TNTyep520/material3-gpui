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
pub struct FabPrimaryContainerTokens;
impl FabPrimaryContainerTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY_CONTAINER;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const FOCUSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const HOVERED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL4;
    pub const HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
}
impl FabPrimaryContainerTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "FabPrimaryContainerTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(FabPrimaryContainerTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "FabPrimaryContainerTokens",
            name: "ContainerElevation",
            value: TokenValue::Dp(FabPrimaryContainerTokens::CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FabPrimaryContainerTokens",
            name: "FocusedContainerElevation",
            value: TokenValue::Dp(FabPrimaryContainerTokens::FOCUSED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FabPrimaryContainerTokens",
            name: "FocusedIconColor",
            value: TokenValue::ColorRole(FabPrimaryContainerTokens::FOCUSED_ICON_COLOR),
        },
        TokenEntry {
            group: "FabPrimaryContainerTokens",
            name: "HoveredContainerElevation",
            value: TokenValue::Dp(FabPrimaryContainerTokens::HOVERED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FabPrimaryContainerTokens",
            name: "HoveredIconColor",
            value: TokenValue::ColorRole(FabPrimaryContainerTokens::HOVERED_ICON_COLOR),
        },
        TokenEntry {
            group: "FabPrimaryContainerTokens",
            name: "IconColor",
            value: TokenValue::ColorRole(FabPrimaryContainerTokens::ICON_COLOR),
        },
        TokenEntry {
            group: "FabPrimaryContainerTokens",
            name: "PressedContainerElevation",
            value: TokenValue::Dp(FabPrimaryContainerTokens::PRESSED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FabPrimaryContainerTokens",
            name: "PressedIconColor",
            value: TokenValue::ColorRole(FabPrimaryContainerTokens::PRESSED_ICON_COLOR),
        },
    ];
}
