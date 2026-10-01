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
pub struct NavigationRailExpandedTokens;
impl NavigationRailExpandedTokens {
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_NONE;
    pub const CONTAINER_WIDTH_MAXIMUM: Dp = Dp(360.0);
    pub const CONTAINER_WIDTH_MINIMUM: Dp = Dp(220.0);
    pub const MODAL_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const MODAL_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const TOP_SPACE: Dp = Dp(44.0);
    pub const MODAL_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER;
}
impl NavigationRailExpandedTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "NavigationRailExpandedTokens",
            name: "ContainerElevation",
            value: TokenValue::Dp(NavigationRailExpandedTokens::CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "NavigationRailExpandedTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(NavigationRailExpandedTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "NavigationRailExpandedTokens",
            name: "ContainerWidthMaximum",
            value: TokenValue::Dp(NavigationRailExpandedTokens::CONTAINER_WIDTH_MAXIMUM),
        },
        TokenEntry {
            group: "NavigationRailExpandedTokens",
            name: "ContainerWidthMinimum",
            value: TokenValue::Dp(NavigationRailExpandedTokens::CONTAINER_WIDTH_MINIMUM),
        },
        TokenEntry {
            group: "NavigationRailExpandedTokens",
            name: "ModalContainerElevation",
            value: TokenValue::Dp(NavigationRailExpandedTokens::MODAL_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "NavigationRailExpandedTokens",
            name: "ModalContainerShape",
            value: TokenValue::ShapeRole(NavigationRailExpandedTokens::MODAL_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "NavigationRailExpandedTokens",
            name: "TopSpace",
            value: TokenValue::Dp(NavigationRailExpandedTokens::TOP_SPACE),
        },
        TokenEntry {
            group: "NavigationRailExpandedTokens",
            name: "ModalContainerColor",
            value: TokenValue::ColorRole(NavigationRailExpandedTokens::MODAL_CONTAINER_COLOR),
        },
    ];
}
